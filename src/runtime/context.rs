use super::class::Class;
use super::descriptor::{Descriptor, MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::method::Method;
use super::native_impl::{self, NativeMethod};
use super::object::Object;
use super::value::Value;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::string::JvmString;

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};

const GC_THRESHOLD: u32 = 4096;

#[derive(Clone, Copy)]
pub struct Context {
    // The function to call into to load resources.
    loader_backend: Gc<Box<dyn ResourceLoader>>,

    // The global class registry.
    class_registry: Gc<RefCell<HashMap<JvmString, Class>>>,

    // Clinits, queued. These cannot be run when loading a class; they will
    // all be run in order every time an Interpreter is started. Yes, this is
    // actually correct, it doesn't matter how wrong it sounds.
    queued_clinits: Gc<RefCell<VecDeque<Method>>>,

    // A map of class T to the object of type Class<T>.
    class_to_object_map: Gc<RefCell<HashMap<Class, Object>>>,

    // A list of JAR files to check for classes.
    jar_files: Gc<RefCell<Vec<Jar>>>,

    // Native method mappings
    native_mapping: Gc<RefCell<HashMap<(JvmString, JvmString, MethodDescriptor), NativeMethod>>>,

    // Values currently in locals or stacks of interpreter frames
    pub frame_data: Gc<RefCell<Box<[Cell<Value>]>>>,

    // The first unoccupied frame data index
    pub frame_index: Gc<Cell<usize>>,

    // The GC counter. This is incremented when any op that could allocate is run,
    // and when it reaches GC_THRESHOLD, a collection is called.
    gc_counter: Gc<Cell<u32>>,

    // Common strings and descriptors.
    pub common: CommonData,

    // The GC context.
    pub gc_ctx: GcCtx,
}

const GLOBALS_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.jar"));

impl Context {
    pub fn new(gc_ctx: GcCtx, loader_backend: Box<dyn ResourceLoader>) -> Self {
        let empty_frame_data = vec![Cell::new(Value::Integer(0)); 80000].into_boxed_slice();

        let created_self = Self {
            loader_backend: Gc::new(gc_ctx, loader_backend),
            class_registry: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            queued_clinits: Gc::new(gc_ctx, RefCell::new(VecDeque::new())),
            class_to_object_map: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            jar_files: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            native_mapping: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            frame_data: Gc::new(gc_ctx, RefCell::new(empty_frame_data)),
            frame_index: Gc::new(gc_ctx, Cell::new(0)),
            gc_counter: Gc::new(gc_ctx, Cell::new(0)),
            common: CommonData::new(gc_ctx),
            gc_ctx,
        };

        let globals_jar =
            Jar::from_bytes(gc_ctx, GLOBALS_JAR.to_vec()).expect("Builtin globals should be valid");
        created_self.add_jar(globals_jar);

        created_self.register_native_mapping();

        created_self
    }

    fn register_native_mapping(self) {
        #[rustfmt::skip]
        let mappings: &[(&str, NativeMethod)] = &[
            ("java/io/PrintStream.stringToUtf8.(Ljava/lang/String;)[B", native_impl::string_to_utf8),
            ("java/lang/StdoutStream.write.(I)V", native_impl::stdout_write),
            ("java/lang/StderrStream.write.(I)V", native_impl::stderr_write),
            ("java/lang/System.arraycopy.(Ljava/lang/Object;ILjava/lang/Object;II)V", native_impl::array_copy),
            ("java/lang/Class.isInterface.()Z", native_impl::is_interface),
            ("java/lang/Object.getClass.()Ljava/lang/Class;", native_impl::get_class),
            ("java/lang/Class.getNameNative.()Ljava/lang/String;", native_impl::get_name_native),
            ("java/lang/System.exit.(I)V;", native_impl::system_exit),
            ("java/lang/Class.getResourceData.(Ljava/lang/String;)[B", native_impl::get_resource_data),
        ];

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

        if let Some(class) = class_registry.get(&class_name) {
            Ok(*class)
        } else if class_name.starts_with('[') {
            drop(class_registry);
            let array_descriptor = Descriptor::from_string(self.gc_ctx, class_name)
                .ok_or_else(|| self.no_class_def_found_error())?;

            let inner_descriptor = array_descriptor.array_inner_descriptor().unwrap();
            let resolved_descriptor = ResolvedDescriptor::from_descriptor(self, inner_descriptor)?;

            let created_class = Class::for_array(self, resolved_descriptor);
            self.register_class(created_class);

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

            Err(self.no_class_def_found_error())
        }
    }

    pub fn register_class(self, class: Class) {
        let class_name = class.name();
        let mut registry = self.class_registry.borrow_mut();

        if registry.contains_key(&class_name) {
            panic!("Attempted to register class {} twice", class_name);
        } else {
            registry.insert(class_name, class);
        }
    }

    pub fn queue_clinit(self, clinit: Method) {
        self.queued_clinits.borrow_mut().push_back(clinit);
    }

    // Should only be run from Interpreter::interpret_ops
    pub fn run_clinits(self) -> Result<(), Error> {
        // Note that running a clinit can queue more clinits
        let mut clinits_copy = self.queued_clinits.borrow_mut().clone();
        self.queued_clinits.borrow_mut().clear();

        while let Some(clinit) = clinits_copy.pop_front() {
            clinit.exec(self, &[])?;
        }

        Ok(())
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

    pub fn no_class_def_found_error(&self) -> Error {
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
        self.queued_clinits.trace();
        self.class_to_object_map.trace();
        self.jar_files.trace();
        self.native_mapping.trace();

        // We want to do a custom tracing over frame data to avoid tracing values
        // above frame_index. This approach isn't too hacky and works well.
        self.frame_data.trace_self();
        let data = self.frame_data.borrow();
        let mut i = 0;
        while i < self.frame_index.get() {
            data[i].trace();
            i += 1;
        }

        self.frame_index.trace();

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
    pub java_lang_class_cast_exception: JvmString,
    pub java_lang_no_class_def_found_error: JvmString,
    pub java_lang_null_pointer_exception: JvmString,

    pub array_byte_desc: JvmString,
    pub array_char_desc: JvmString,
    pub array_int_desc: JvmString,
    pub array_long_desc: JvmString,

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
            java_lang_class_cast_exception: JvmString::new(
                gc_ctx,
                "java/lang/ClassCastException".to_string(),
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
        self.java_lang_class_cast_exception.trace();
        self.java_lang_no_class_def_found_error.trace();
        self.java_lang_null_pointer_exception.trace();

        self.array_byte_desc.trace();
        self.array_char_desc.trace();
        self.array_int_desc.trace();
        self.array_long_desc.trace();

        self.init_name.trace();
        self.clinit_name.trace();

        self.noargs_void_desc.trace();
        self.arg_char_array_void_desc.trace();
    }
}

#[derive(Clone)]
pub enum ResourceLoadType {
    // This class was loaded directly from the filesystem. When searching
    // for resources, look at the files in the directory of this class.
    FileSystem,

    // This class was loaded from a JAR file. When searching for resources,
    // look at the files in the directory of this class in the JAR.
    Jar(Jar),
}

impl Trace for ResourceLoadType {
    fn trace(&self) {
        match self {
            ResourceLoadType::Jar(jar) => jar.trace(),
            _ => {}
        }
    }
}

pub trait ResourceLoader {
    fn load_resource(
        &self,
        load_type: &ResourceLoadType,
        class_name: &String,
        resource_name: &String,
    ) -> Option<Vec<u8>>;
}
