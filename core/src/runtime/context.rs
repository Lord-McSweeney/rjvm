use super::builtins::{BuiltinClasses, PrimitiveArrayClasses};
use super::call_stack::CallStack;
use super::class::{Class, PrimitiveType};
use super::descriptor::MethodDescriptor;
use super::error::Error;
use super::intern::InternedStrings;
use super::loader::{ClassLoader, LoaderBackend, ResourceLoadSource};
use super::method::{Method, NativeMethod};
use super::object::Object;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::string::JvmString;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{Cell, OnceCell, Ref, RefCell};
use hashbrown::HashMap;

// Various magic fields
pub const OBJECT_TO_STRING_METHOD: usize = 2;

pub const THROWABLE_MESSAGE_FIELD: usize = 0;
pub const THROWABLE_STACK_TRACE_FIELD: usize = 1;

pub const STRING_DATA_FIELD: usize = 0;

pub const STACK_TRACE_ELEMENT_CREATE_METHOD: usize = 0;

const DEFAULT_GC_THRESHOLD: u32 = 131072;

#[derive(Clone)]
pub struct Context {
    // The backend to call into to load resources.
    pub loader_backend: Gc<Box<dyn LoaderBackend>>,

    // The "bootstrap" class loader
    bootstrap_loader: ClassLoader,

    // The "system" (aka "application") class loader
    system_loader: Gc<OnceCell<ClassLoader>>,

    // A list of classes that have an associated `java.lang.Class`. Java code
    // stores an index into this array.
    java_classes: Gc<RefCell<Vec<Class>>>,

    // A list of methods that have an associated `java.lang.reflect.Method`.
    // Java code stores an index into this array.
    java_executables: Gc<RefCell<Vec<Method>>>,

    // A list of class loaders that have an associated `java.lang.ClassLoader`.
    // Java code stores an index into this array.
    java_class_loaders: Gc<RefCell<Vec<ClassLoader>>>,

    // All interned Java String objects.
    interned_strings: Gc<RefCell<InternedStrings>>,

    // The builtin primitive classes, constructed on JVM startup.
    primitive_classes: Gc<HashMap<PrimitiveType, Class>>,

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

    // The number of allocation operations before the GC runs.
    gc_threshold: Gc<Cell<u32>>,

    // The class `java.lang.Object`. This is critical for all class loading,
    // so we store it separately.
    object_class: Gc<OnceCell<Class>>,

    // Builtin classes, such as `NoClassDefFoundError`, that the VM needs to
    // access quickly
    builtins: Gc<RefCell<Option<BuiltinClasses>>>,

    // Like `builtins`, but for the primitive array classes.
    primitive_arrays: Gc<RefCell<Option<PrimitiveArrayClasses>>>,

    // Common strings and descriptors.
    pub common: CommonData,

    // The GC context.
    pub gc_ctx: GcCtx,
}

impl Context {
    pub fn new(loader_backend: Box<dyn LoaderBackend>) -> Self {
        let gc_ctx = GcCtx::new();

        let empty_frame_data = vec![Cell::new(Value::Integer(0)); 80000].into_boxed_slice();

        let mut primitive_classes = HashMap::new();
        let primitive_types = PrimitiveType::get_all();
        for primitive_type in primitive_types {
            // Create a primitive class for each primitive class
            primitive_classes.insert(primitive_type, Class::for_primitive(gc_ctx, primitive_type));
        }

        let loader_backend = Gc::new(gc_ctx, loader_backend);
        let bootstrap_loader = ClassLoader::bootstrap(gc_ctx, loader_backend);

        Self {
            loader_backend,
            bootstrap_loader,
            system_loader: Gc::new(gc_ctx, OnceCell::new()),
            java_classes: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            java_executables: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            java_class_loaders: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            interned_strings: Gc::new(gc_ctx, RefCell::new(InternedStrings::new())),
            primitive_classes: Gc::new(gc_ctx, primitive_classes),
            native_mapping: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            frame_data: Gc::new(gc_ctx, empty_frame_data),
            frame_index: Gc::new(gc_ctx, Cell::new(0)),
            call_stack: Gc::new(gc_ctx, RefCell::new(CallStack::empty())),
            gc_counter: Gc::new(gc_ctx, Cell::new(0)),
            gc_threshold: Gc::new(gc_ctx, Cell::new(DEFAULT_GC_THRESHOLD)),
            object_class: Gc::new(gc_ctx, OnceCell::new()),
            builtins: Gc::new(gc_ctx, RefCell::new(None)),
            primitive_arrays: Gc::new(gc_ctx, RefCell::new(None)),
            common: CommonData::new(gc_ctx),
            gc_ctx,
        }
    }

    pub fn register_native_mappings(&self, mappings: &[(&str, NativeMethod)]) {
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

    pub fn set_gc_threshold(&self, gc_threshold: u32) {
        self.gc_threshold.set(gc_threshold);
    }

    pub fn get_native_method(
        &self,
        class_name: JvmString,
        method_name: JvmString,
        method_descriptor: MethodDescriptor,
    ) -> Option<NativeMethod> {
        self.native_mapping
            .borrow()
            .get(&(class_name, method_name, method_descriptor))
            .copied()
    }

    pub fn add_bootstrap_jar(&self, jar: Jar) {
        self.bootstrap_loader
            .add_source(ResourceLoadSource::Jar(jar));
    }

    pub fn add_system_jar(&self, jar: Jar) {
        self.system_loader()
            .add_source(ResourceLoadSource::Jar(jar));
    }

    pub fn bootstrap_loader(&self) -> ClassLoader {
        self.bootstrap_loader
    }

    pub fn system_loader(&self) -> ClassLoader {
        *self
            .system_loader
            .get()
            .expect("Attempted to access system loader before it was initialized")
    }

    pub fn init_system_loader(&self, loader: ClassLoader) {
        self.system_loader
            .set(loader)
            .expect("Attempted to set system loader after was initialized");
    }

    pub fn add_class_object(&self, class: Class) -> i32 {
        let mut borrow = self.java_classes.borrow_mut();
        borrow.push(class);

        borrow.len() as i32 - 1
    }

    pub fn class_object_by_id(&self, id: i32) -> Class {
        self.java_classes.borrow()[id as usize]
    }

    pub fn add_executable_object(&self, method: Method) -> i32 {
        let mut borrow = self.java_executables.borrow_mut();
        borrow.push(method);

        borrow.len() as i32 - 1
    }

    pub fn executable_object_by_id(&self, id: i32) -> Method {
        self.java_executables.borrow()[id as usize]
    }

    pub fn add_class_loader_object(&self, loader: ClassLoader) -> i32 {
        let mut borrow = self.java_class_loaders.borrow_mut();
        borrow.push(loader);

        borrow.len() as i32 - 1
    }

    pub fn class_loader_object_by_id(&self, id: i32) -> ClassLoader {
        self.java_class_loaders.borrow()[id as usize]
    }

    pub fn intern_string_obj(&self, new_string: Object) -> Object {
        self.interned_strings.borrow_mut().intern(new_string)
    }

    pub fn primitive_class_for(&self, primitive_type: PrimitiveType) -> Class {
        *self.primitive_classes.get(&primitive_type).unwrap()
    }

    pub fn call_stack_size(&self) -> usize {
        self.call_stack.borrow().len()
    }

    pub fn push_call(&self, method: Method) {
        self.call_stack.borrow_mut().push_call(method);
    }

    pub fn pop_call(&self) {
        self.call_stack.borrow_mut().pop_call();
    }

    pub fn capture_call_stack(&self) -> Vec<Object> {
        let entries = self.call_stack.borrow().get_entries();
        CallStack::display(self, &entries)
    }

    pub fn increment_gc_counter(&self) {
        let new_value = self.gc_counter.get() + 1;

        if new_value == self.gc_threshold.get() {
            unsafe {
                self.gc_ctx.collect(self);
            }

            self.gc_counter.set(0);
        } else {
            self.gc_counter.set(new_value);
        }
    }

    /// Convert a Java String object to a Rust `String`.
    pub fn string_object_to_string(string_obj: Object) -> String {
        let chars = Context::unwrap_string(string_obj);

        return String::from_utf16_lossy(&chars);
    }

    /// Convert a Rust `JvmString` to a Java String object.
    pub fn jvm_string_to_string(&self, string: JvmString) -> Object {
        let chars = string.chars().map(|c| c as u16).collect::<Box<_>>();

        let chars_array_object = Object::char_array(&self, chars);

        let string_class = self.builtins().java_lang_string;

        let string_instance = string_class.new_instance(self.gc_ctx);
        string_instance.set_field(STRING_DATA_FIELD, Value::Object(Some(chars_array_object)));

        string_instance
    }

    /// Convert a Rust `&[u16]` to a Java String object.
    pub fn create_string(&self, chars: &[u16]) -> Object {
        let chars_array_object = Object::char_array(&self, Box::from(chars));

        let string_class = self.builtins().java_lang_string;

        let string_instance = string_class.new_instance(self.gc_ctx);
        string_instance.set_field(STRING_DATA_FIELD, Value::Object(Some(chars_array_object)));

        string_instance
    }

    /// Convert a Java String object to a Rust `Box<[u16]>`.
    pub fn unwrap_string(string_obj: Object) -> Box<[u16]> {
        let chars = string_obj.get_field(STRING_DATA_FIELD).object().unwrap();
        let chars = chars.array_data().as_char_array();
        let chars = chars.iter().map(|c| c.get()).collect::<Box<_>>();

        chars
    }

    pub fn object_class(&self) -> Class {
        self.object_class
            .get()
            .copied()
            .expect("Builtin classes should have been loaded")
    }

    pub fn builtins(&self) -> Ref<'_, BuiltinClasses> {
        let builtins = self.builtins.borrow();
        Ref::map(builtins, |b| {
            b.as_ref().expect("Builtin classes should have been loaded")
        })
    }

    pub fn primitive_arrays(&self) -> Ref<'_, PrimitiveArrayClasses> {
        let primitive_arrays = self.primitive_arrays.borrow();
        Ref::map(primitive_arrays, |b| {
            b.as_ref()
                .expect("Primitive array classes should have been loaded")
        })
    }

    pub fn load_builtins(&self) {
        // First load the `Object` class
        let object_class_name = "java/lang/Object".to_string();
        let object_class_name = JvmString::new(self.gc_ctx, object_class_name);

        let object_class = self
            .bootstrap_loader()
            .find_class(self, object_class_name)
            .expect("Object class did not parse")
            .expect("Object class did not exist");

        let _ = self.object_class.set(object_class);

        // The primitive array classes require absolutely nothing to initialize,
        // except for `Object`, so get them loaded next
        let builtin_primitive_arrays = PrimitiveArrayClasses::new(self);
        *self.primitive_arrays.borrow_mut() = Some(builtin_primitive_arrays);

        let builtin_classes = BuiltinClasses::new(self);
        *self.builtins.borrow_mut() = Some(builtin_classes);

        // Run the Java-side initialization method, which is the first static
        // method of the `System` class
        let system_class = self.builtins().java_lang_system;
        let init_method = system_class.static_methods()[0];
        init_method
            .exec(self, &[])
            .expect("System initializer method failed");
    }

    pub fn arithmetic_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_arithmetic_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn array_index_oob_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_array_index_oob_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn array_store_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_array_store_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn class_cast_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_class_cast_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn clone_not_supported_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_clone_not_supported_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn illegal_access_error(&self) -> Error {
        let exception_class = self.builtins().java_lang_illegal_access_error;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn instantiation_error(&self) -> Error {
        let exception_class = self.builtins().java_lang_instantiation_error;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn instantiation_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_instantiation_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn negative_array_size_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_negative_array_size_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(exception_instance))],
            )
            .expect("Exception class should construct");

        Error::Java(exception_instance)
    }

    pub fn no_class_def_found_error(&self, class_name: JvmString) -> Error {
        let error_class = self.builtins().java_lang_no_class_def_found_error;

        let error_instance = error_class.new_instance(self.gc_ctx);
        error_instance
            .call_construct(
                &self,
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

    pub fn no_such_field_error(&self) -> Error {
        let error_class = self.builtins().java_lang_no_such_field_error;

        let error_instance = error_class.new_instance(self.gc_ctx);
        error_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(error_instance))],
            )
            .expect("Error class should construct");

        Error::Java(error_instance)
    }

    pub fn no_such_method_error(&self) -> Error {
        let error_class = self.builtins().java_lang_no_such_method_error;

        let error_instance = error_class.new_instance(self.gc_ctx);
        error_instance
            .call_construct(
                &self,
                self.common.noargs_void_desc,
                &[Value::Object(Some(error_instance))],
            )
            .expect("Error class should construct");

        Error::Java(error_instance)
    }

    pub fn null_pointer_exception(&self) -> Error {
        let exception_class = self.builtins().java_lang_null_pointer_exception;

        let exception_instance = exception_class.new_instance(self.gc_ctx);
        exception_instance
            .call_construct(
                &self,
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

        self.bootstrap_loader.trace();
        self.system_loader.trace();
        self.java_classes.trace();
        self.java_executables.trace();
        self.java_class_loaders.trace();
        self.interned_strings.trace();
        self.primitive_classes.trace();
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
        self.gc_threshold.trace();

        self.object_class.trace();
        self.builtins.trace();
        self.primitive_arrays.trace();
        self.common.trace();
    }
}

#[derive(Clone, Copy)]
pub struct CommonData {
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
            init_name: JvmString::new(gc_ctx, "<init>".to_string()),
            clinit_name: JvmString::new(gc_ctx, "<clinit>".to_string()),
            noargs_void_desc,
            arg_char_array_void_desc,
        }
    }
}

impl Trace for CommonData {
    fn trace(&self) {
        self.init_name.trace();
        self.clinit_name.trace();

        self.noargs_void_desc.trace();
        self.arg_char_array_void_desc.trace();
    }
}
