use super::class::Class;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};

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

    pub common: CommonData,

    pub gc_ctx: GcCtx,
}

const GLOBALS_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.jar"));

impl Context {
    pub fn new(gc_ctx: GcCtx) -> Self {
        let created_self = Self {
            class_registry: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            jar_files: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            common: CommonData::new(gc_ctx),
            gc_ctx,
        };

        created_self.init_object_class();

        let globals_jar =
            Jar::from_bytes(gc_ctx, GLOBALS_JAR.to_vec()).expect("Builtin globals should be valid");
        created_self.jar_files.borrow_mut().push(globals_jar);

        created_self
    }

    fn init_object_class(self) {
        let object_class = Class::create_object_class(self);

        self.register_class(object_class);
    }

    pub fn lookup_class(self, class_name: JvmString) -> Result<Class, Error> {
        let class_registry = self.class_registry.borrow();

        if let Some(class) = class_registry.get(&class_name) {
            Ok(*class)
        } else if class_name.starts_with('[') {
            drop(class_registry);
            let array_descriptor = Descriptor::from_string(self.gc_ctx, class_name)
                .ok_or(Error::Native(NativeError::ClassNotFound))?;

            let created_class = Class::for_array(self, array_descriptor);
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

            Err(Error::Native(NativeError::ClassNotFound))
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
}

impl Trace for Context {
    fn trace(&self) {
        self.class_registry.trace();
        self.jar_files.trace();
        self.class_registry.borrow().trace();
    }
}

#[derive(Clone, Copy)]
pub struct CommonData {
    pub java_lang_object: JvmString,
    pub java_lang_string: JvmString,
    pub array_char_desc: JvmString,
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
            array_char_desc: JvmString::new(gc_ctx, "[C".to_string()),
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
        self.array_char_desc.trace();
        self.init_name.trace();
        self.clinit_name.trace();
        self.noargs_void_desc.trace();
        self.arg_char_array_void_desc.trace();
    }
}
