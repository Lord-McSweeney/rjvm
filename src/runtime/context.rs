use super::class::Class;
use super::descriptor::{Descriptor, MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::native_impl::{self, NativeMethod};
use super::value::Value;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::string::JvmString;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Context {
    class_registry: Gc<RefCell<HashMap<JvmString, Class>>>,

    jar_files: Gc<RefCell<Vec<Jar>>>,

    native_mapping: Gc<RefCell<HashMap<(JvmString, JvmString, MethodDescriptor), NativeMethod>>>,

    pub common: CommonData,

    pub gc_ctx: GcCtx,
}

const GLOBALS_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.jar"));

impl Context {
    pub fn new(gc_ctx: GcCtx) -> Self {
        let created_self = Self {
            class_registry: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            jar_files: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            native_mapping: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
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
        // java/io/PrintStream : static byte[] stringToUtf8(String)
        {
            let printstream_name = JvmString::new(self.gc_ctx, "java/io/PrintStream".to_string());

            let method_name = JvmString::new(self.gc_ctx, "stringToUtf8".to_string());

            let descriptor_name = JvmString::new(self.gc_ctx, "(Ljava/lang/String;)[B".to_string());
            let descriptor = MethodDescriptor::from_string(self.gc_ctx, descriptor_name)
                .expect("Valid descriptor");

            self.native_mapping.borrow_mut().insert(
                (printstream_name, method_name, descriptor),
                native_impl::string_to_utf8,
            );
        }

        // java/lang/StdoutStream : void write(int)
        {
            let stdoutstream_name =
                JvmString::new(self.gc_ctx, "java/lang/StdoutStream".to_string());

            let method_name = JvmString::new(self.gc_ctx, "write".to_string());

            let descriptor_name = JvmString::new(self.gc_ctx, "(I)V".to_string());
            let descriptor = MethodDescriptor::from_string(self.gc_ctx, descriptor_name)
                .expect("Valid descriptor");

            self.native_mapping.borrow_mut().insert(
                (stdoutstream_name, method_name, descriptor),
                native_impl::stdout_write,
            );
        }

        // java/lang/System : static void arraycopy(Object, int, Object, int, int)
        {
            let system_name = JvmString::new(self.gc_ctx, "java/lang/System".to_string());

            let method_name = JvmString::new(self.gc_ctx, "arraycopy".to_string());

            let descriptor_name = JvmString::new(
                self.gc_ctx,
                "(Ljava/lang/Object;ILjava/lang/Object;II)V".to_string(),
            );
            let descriptor = MethodDescriptor::from_string(self.gc_ctx, descriptor_name)
                .expect("Valid descriptor");

            self.native_mapping.borrow_mut().insert(
                (system_name, method_name, descriptor),
                native_impl::array_copy,
            );
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
                .ok_or(self.no_class_def_found_error())?;

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

                    let class = Class::from_class_file(self, class_file)?;
                    self.register_class(class);
                    class.load_method_data(self)?;

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
            panic!("Attempted to register class under name that other class was already registered under");
        } else {
            registry.insert(class_name, class);
        }
    }

    pub fn add_linked_jar(self, jar: Jar) {
        self.jar_files.borrow_mut().push(jar);
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
        self.class_registry.trace();
        self.jar_files.trace();
        self.class_registry.borrow().trace();

        for k in self.native_mapping.borrow().keys() {
            k.0.trace();
            k.1.trace();
            k.2.trace();
        }
    }
}

#[derive(Clone, Copy)]
pub struct CommonData {
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
            init_name: JvmString::new(gc_ctx, "<init>".to_string()),
            clinit_name: JvmString::new(gc_ctx, "<clinit>".to_string()),
            noargs_void_desc,
            arg_char_array_void_desc,
        }
    }
}

impl Trace for CommonData {
    fn trace(&self) {
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

        self.init_name.trace();
        self.clinit_name.trace();

        self.noargs_void_desc.trace();
        self.arg_char_array_void_desc.trace();
    }
}
