use super::call_stack::CallStack;
use super::class::Class;
use super::descriptor::{Descriptor, MethodDescriptor, ResolvedDescriptor};
use super::error::Error;
use super::loader::{LoaderBackend, ResourceLoadType};
use super::method::{Method, NativeMethod};
use super::object::Object;
use super::value::Value;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::string::JvmString;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

// Various magic fields
pub const OBJECT_TO_STRING_METHOD: usize = 2;

pub const THROWABLE_MESSAGE_FIELD: usize = 0;
pub const THROWABLE_STACK_TRACE_FIELD: usize = 1;

pub const STRING_DATA_FIELD: usize = 0;

/// The number of allocation operations before the GC runs.
const GC_THRESHOLD: u32 = 32768;

#[derive(Clone, Copy)]
pub struct Context {
    // The backend to call into to load resources.
    loader_backend: Gc<Box<dyn LoaderBackend>>,

    // The global class registry. This is stored as a hashmap in list form; it
    // is assumed that classes will never be removed, so we can create integer
    // ids to classes.
    class_registry: Gc<RefCell<Vec<(JvmString, Class)>>>,

    // A map of class T to the object of type Class<T>.
    class_to_object_map: Gc<RefCell<HashMap<Class, Object>>>,

    // A map of descriptor D to class [D.
    array_classes: Gc<RefCell<HashMap<ResolvedDescriptor, Class>>>,

    // A list of JAR files to check for classes.
    jar_files: Gc<RefCell<Vec<Jar>>>,

    // Native method mappings
    native_mapping: Gc<RefCell<HashMap<(JvmString, JvmString, MethodDescriptor), NativeMethod>>>,

    // Values currently in locals or stacks of interpreter frames
    pub frame_data: Gc<Box<[Cell<Value>]>>,

    // The first index into the frame data that is unoccupied (stack pointer).
    pub frame_index: Gc<Cell<usize>>,

    // The current call stack.
    call_stack: Gc<RefCell<CallStack>>,

    // The GC counter. This is incremented when any op that could allocate is run,
    // and when it reaches GC_THRESHOLD, a collection is called.
    gc_counter: Gc<Cell<u32>>,

    // Common strings and descriptors.
    pub common: CommonData,

    // The GC context.
    pub gc_ctx: GcCtx,
}

impl Context {
    pub fn new(loader_backend: Box<dyn LoaderBackend>) -> Self {
        let gc_ctx = GcCtx::new();

        let empty_frame_data = vec![Cell::new(Value::Integer(0)); 80000].into_boxed_slice();

        Self {
            loader_backend: Gc::new(gc_ctx, loader_backend),
            class_registry: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            class_to_object_map: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            array_classes: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            jar_files: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            native_mapping: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            frame_data: Gc::new(gc_ctx, empty_frame_data),
            frame_index: Gc::new(gc_ctx, Cell::new(0)),
            call_stack: Gc::new(gc_ctx, RefCell::new(CallStack::empty())),
            gc_counter: Gc::new(gc_ctx, Cell::new(0)),
            common: CommonData::new(gc_ctx),
            gc_ctx,
        }
    }

    pub fn register_native_mappings(self, mappings: &[(&str, NativeMethod)]) {
        for mapping in mappings {
            let name = mapping.0.split(".").collect::<Vec<_>>();

            let class_name = JvmString::new(self.gc_ctx, name[0].to_string());
            let method_name = JvmString::new(self.gc_ctx, name[1].to_string());
            let descriptor_name = JvmString::new(self.gc_ctx, name[2].to_string());

            let descriptor = MethodDescriptor::from_string(self.gc_ctx, descriptor_name)
                .expect("Valid descriptor");

            let method = mapping.1;

            self.native_mapping
                .borrow_mut()
                .insert((class_name, method_name, descriptor), method);
        }
    }

    pub fn get_native_method(
        self,
        class_name: JvmString,
        method_name: JvmString,
        method_descriptor: MethodDescriptor,
    ) -> Option<NativeMethod> {
        self.native_mapping
            .borrow()
            .get(&(class_name, method_name, method_descriptor))
            .copied()
    }

    pub fn add_jar(self, jar: Jar) {
        self.jar_files.borrow_mut().push(jar);
    }

    pub fn lookup_class(self, class_name: JvmString) -> Result<Class, Error> {
        let class_registry = self.class_registry.borrow();

        if let Some((_, class)) = class_registry.iter().find(|(name, _)| name == &class_name) {
            Ok(*class)
        } else if let Some(element_name) = class_name.strip_prefix('[') {
            let element_name = JvmString::new(self.gc_ctx, element_name.to_string());
            drop(class_registry);
            let element_descriptor = Descriptor::from_string(self.gc_ctx, element_name)
                .ok_or_else(|| self.no_class_def_found_error(class_name))?;

            let resolved_descriptor =
                ResolvedDescriptor::from_descriptor(self, element_descriptor)?;

            let created_class = self.array_class_for(resolved_descriptor);
            // `array_class_for` will register the class in the registry

            Ok(created_class)
        } else {
            drop(class_registry);

            for jar_file in &*self.jar_files.borrow() {
                if jar_file.has_class(class_name) {
                    let read_data = jar_file.read_class(class_name)?;

                    let class_file = ClassFile::from_data(self.gc_ctx, read_data)?;

                    let class =
                        Class::from_class_file(self, ResourceLoadType::Jar(*jar_file), class_file)?;

                    self.register_class(class);

                    class.load_methods(self)?;

                    return Ok(class);
                }
            }

            Err(self.no_class_def_found_error(class_name))
        }
    }

    pub fn register_class(self, class: Class) {
        let class_name = class.name();
        let mut registry = self.class_registry.borrow_mut();

        if registry.iter().any(|(name, _)| name == &class_name) {
            panic!("Attempted to register class {} twice", class_name);
        } else {
            registry.push((class_name, class));
        }
    }

    pub fn class_id_by_class(self, searched_class: Class) -> usize {
        let registry = self.class_registry.borrow();
        for (i, element) in registry.iter().enumerate() {
            let (_, class) = element;
            if *class == searched_class {
                return i;
            }
        }

        panic!("class_id_by_class expects a registered class")
    }

    pub fn class_by_class_id(self, searched_id: usize) -> Class {
        let registry = self.class_registry.borrow();
        if let Some((_, class)) = registry.get(searched_id) {
            *class
        } else {
            panic!("class_by_class_id expects a valid class id")
        }
    }

    pub fn add_linked_jar(self, jar: Jar) {
        self.jar_files.borrow_mut().push(jar);
    }

    pub fn load_resource(
        self,
        load_type: &ResourceLoadType,
        class_name: &String,
        resource_name: &String,
    ) -> Option<Vec<u8>> {
        self.loader_backend
            .load_resource(load_type, class_name, resource_name)
    }

    pub fn class_object_for_class(self, class: Class) -> Object {
        let mut class_objects = self.class_to_object_map.borrow_mut();

        if let Some(class_object) = class_objects.get(&class) {
            *class_object
        } else {
            let object = Object::class_object(self, class);

            class_objects.insert(class, object);

            object
        }
    }

    // Used to avoid making multiple array classes for one array type
    pub fn array_class_for(self, descriptor: ResolvedDescriptor) -> Class {
        let array_classes = self.array_classes.borrow();
        if let Some(class) = array_classes.get(&descriptor) {
            *class
        } else {
            drop(array_classes);
            let created_class = Class::for_array(self, descriptor);
            self.array_classes
                .borrow_mut()
                .insert(descriptor, created_class);
            self.register_class(created_class);
            created_class
        }
    }

    pub fn push_call(self, method: Method) {
        self.call_stack.borrow_mut().push_call(method);
    }

    pub fn pop_call(self) {
        self.call_stack.borrow_mut().pop_call();
    }

    pub fn capture_call_stack(self) -> String {
        self.call_stack.borrow().display()
    }

    pub fn increment_gc_counter(self) {
        let new_value = self.gc_counter.get() + 1;

        if new_value == GC_THRESHOLD {
            unsafe {
                self.gc_ctx.collect(&self);
            }

            self.gc_counter.set(0);
        } else {
            self.gc_counter.set(new_value);
        }
    }

    /// Convert a Java String object to a Rust `String`.
    pub fn string_object_to_string(string_obj: Object) -> String {
        let chars = string_obj.get_field(STRING_DATA_FIELD).object().unwrap();
        let chars = chars.get_array_data();
        let chars = chars
            .iter()
            .map(|c| c.get().int() as u16)
            .collect::<Vec<_>>();

        return String::from_utf16_lossy(&chars);
    }

    /// Convert a Rust `JvmString` to a Java String object.
    pub fn jvm_string_to_string(self, string: JvmString) -> Object {
        let chars = string.chars().map(|c| c as u16).collect::<Vec<_>>();

        let chars_array_object = Object::char_array(self, &chars);

        let string_class = self
            .lookup_class(self.common.java_lang_string)
            .expect("String class should exist");

        let string_instance = string_class.new_instance(self.gc_ctx);
        string_instance.set_field(STRING_DATA_FIELD, Value::Object(Some(chars_array_object)));

        string_instance
    }

    /// Convert a Rust `&[u16]` to a Java String object.
    pub fn create_string(self, chars: &[u16]) -> Object {
        let chars_array_object = Object::char_array(self, chars);

        let string_class = self
            .lookup_class(self.common.java_lang_string)
            .expect("String class should exist");

        let string_instance = string_class.new_instance(self.gc_ctx);
        string_instance.set_field(STRING_DATA_FIELD, Value::Object(Some(chars_array_object)));

        string_instance
    }

    pub fn arithmetic_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_arithmetic_exception)
            .expect("ArithmeticException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn array_index_oob_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_array_index_oob_exception)
            .expect("ArrayIndexOutOfBoundsException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn array_store_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_array_store_exception)
            .expect("ArrayStoreException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn class_cast_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_class_cast_exception)
            .expect("ClassCastException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn clone_not_supported_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_clone_not_supported_exception)
            .expect("CloneNotSupportedException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn negative_array_size_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_negative_array_size_exception)
            .expect("NegativeArraySizeException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn no_class_def_found_error(&self, class_name: JvmString) -> Error {
        let error_class = self
            .lookup_class(self.common.java_lang_no_class_def_found_error)
            .expect("NoClassDefFoundError class should exist");

        let error_instance = error_class.new_instance(self.gc_ctx);
        error_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(error_instance))],
            )
            .expect("Error class should construct");

        // Set the `message` field
        error_instance.set_field(
            THROWABLE_MESSAGE_FIELD,
            Value::Object(Some(self.jvm_string_to_string(class_name))),
        );

        Error::Java(error_instance)
    }

    pub fn null_pointer_exception(&self) -> Error {
        let exception_class = self
            .lookup_class(self.common.java_lang_null_pointer_exception)
            .expect("NullPointerException class should exist");

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                *self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }
}

impl Trace for Context {
    fn trace(&self) {
        self.loader_backend.trace_self();

        self.class_registry.trace();
        self.class_to_object_map.trace();
        self.array_classes.trace();
        self.jar_files.trace();
        self.native_mapping.trace();

        // We want to do a custom tracing over frame data to avoid tracing values
        // above frame_index. This approach isn't too hacky and works well.
        self.frame_data.trace_self();
        let data = &**self.frame_data;
        let mut i = 0;
        while i < self.frame_index.get() {
            data[i].trace();
            i += 1;
        }

        self.frame_index.trace();

        self.call_stack.trace();

        self.gc_counter.trace();

        self.common.trace();
    }
}

#[derive(Clone, Copy)]
pub struct CommonData {
    pub java_lang_class: JvmString,
    pub java_lang_object: JvmString,
    pub java_lang_string: JvmString,
    pub java_lang_throwable: JvmString,

    pub java_lang_arithmetic_exception: JvmString,
    pub java_lang_array_index_oob_exception: JvmString,
    pub java_lang_array_store_exception: JvmString,
    pub java_lang_class_cast_exception: JvmString,
    pub java_lang_clone_not_supported_exception: JvmString,
    pub java_lang_cloneable: JvmString,
    pub java_lang_negative_array_size_exception: JvmString,
    pub java_lang_no_class_def_found_error: JvmString,
    pub java_lang_null_pointer_exception: JvmString,

    pub array_byte_desc: JvmString,
    pub array_char_desc: JvmString,
    pub array_int_desc: JvmString,
    pub array_long_desc: JvmString,
    pub array_bool_desc: JvmString,

    pub init_name: JvmString,
    pub clinit_name: JvmString,

    pub noargs_void_desc: MethodDescriptor,
    pub arg_char_array_void_desc: MethodDescriptor,
}

impl CommonData {
    fn new(gc_ctx: GcCtx) -> Self {
        let void_descriptor_name = JvmString::new(gc_ctx, "()V".to_string());

        let noargs_void_desc =
            MethodDescriptor::from_string(gc_ctx, void_descriptor_name).expect("Valid descriptor");

        let arg_char_array_void_descriptor_name = JvmString::new(gc_ctx, "([C)V".to_string());

        let arg_char_array_void_desc =
            MethodDescriptor::from_string(gc_ctx, arg_char_array_void_descriptor_name)
                .expect("Valid descriptor");

        Self {
            java_lang_class: JvmString::new(gc_ctx, "java/lang/Class".to_string()),
            java_lang_object: JvmString::new(gc_ctx, "java/lang/Object".to_string()),
            java_lang_string: JvmString::new(gc_ctx, "java/lang/String".to_string()),
            java_lang_throwable: JvmString::new(gc_ctx, "java/lang/Throwable".to_string()),
            java_lang_arithmetic_exception: JvmString::new(
                gc_ctx,
                "java/lang/ArithmeticException".to_string(),
            ),
            java_lang_array_index_oob_exception: JvmString::new(
                gc_ctx,
                "java/lang/ArrayIndexOutOfBoundsException".to_string(),
            ),
            java_lang_array_store_exception: JvmString::new(
                gc_ctx,
                "java/lang/ArrayStoreException".to_string(),
            ),
            java_lang_class_cast_exception: JvmString::new(
                gc_ctx,
                "java/lang/ClassCastException".to_string(),
            ),
            java_lang_clone_not_supported_exception: JvmString::new(
                gc_ctx,
                "java/lang/CloneNotSupportedException".to_string(),
            ),
            java_lang_cloneable: JvmString::new(gc_ctx, "java/lang/Cloneable".to_string()),
            java_lang_negative_array_size_exception: JvmString::new(
                gc_ctx,
                "java/lang/NegativeArraySizeException".to_string(),
            ),
            java_lang_no_class_def_found_error: JvmString::new(
                gc_ctx,
                "java/lang/NoClassDefFoundError".to_string(),
            ),
            java_lang_null_pointer_exception: JvmString::new(
                gc_ctx,
                "java/lang/NullPointerException".to_string(),
            ),
            array_byte_desc: JvmString::new(gc_ctx, "[B".to_string()),
            array_char_desc: JvmString::new(gc_ctx, "[C".to_string()),
            array_int_desc: JvmString::new(gc_ctx, "[I".to_string()),
            array_long_desc: JvmString::new(gc_ctx, "[J".to_string()),
            array_bool_desc: JvmString::new(gc_ctx, "[Z".to_string()),
            init_name: JvmString::new(gc_ctx, "<init>".to_string()),
            clinit_name: JvmString::new(gc_ctx, "<clinit>".to_string()),
            noargs_void_desc,
            arg_char_array_void_desc,
        }
    }
}

impl Trace for CommonData {
    fn trace(&self) {
        self.java_lang_class.trace();
        self.java_lang_object.trace();
        self.java_lang_string.trace();
        self.java_lang_throwable.trace();

        self.java_lang_arithmetic_exception.trace();
        self.java_lang_array_index_oob_exception.trace();
        self.java_lang_array_store_exception.trace();
        self.java_lang_class_cast_exception.trace();
        self.java_lang_clone_not_supported_exception.trace();
        self.java_lang_cloneable.trace();
        self.java_lang_negative_array_size_exception.trace();
        self.java_lang_no_class_def_found_error.trace();
        self.java_lang_null_pointer_exception.trace();

        self.array_byte_desc.trace();
        self.array_char_desc.trace();
        self.array_int_desc.trace();
        self.array_long_desc.trace();
        self.array_bool_desc.trace();

        self.init_name.trace();
        self.clinit_name.trace();

        self.noargs_void_desc.trace();
        self.arg_char_array_void_desc.trace();
    }
}
