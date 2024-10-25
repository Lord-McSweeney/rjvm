use super::class::Class;
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

    pub gc_ctx: GcCtx,
}

const GLOBALS_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.jar"));

impl Context {
    pub fn new(gc_ctx: GcCtx) -> Self {
        let created_self = Self {
            class_registry: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            jar_files: Gc::new(gc_ctx, RefCell::new(Vec::new())),
            gc_ctx,
        };

        created_self.init_object_class();

        let globals_jar =
            Jar::from_bytes(gc_ctx, GLOBALS_JAR.to_vec()).expect("Builtin globals should be valid");
        created_self.jar_files.borrow_mut().push(globals_jar);

        created_self
    }

    fn init_object_class(self) {
        let object_class_name = JvmString::new(self.gc_ctx, "java/lang/Object".to_string());
        let object_class = Class::new(self.gc_ctx, object_class_name);

        self.register_class(object_class);
    }

    pub fn lookup_class(self, class_name: JvmString) -> Result<Class, Error> {
        let class_registry = self.class_registry.borrow();

        if let Some(class) = class_registry.get(&class_name) {
            Ok(*class)
        } else {
            drop(class_registry);

            for jar_file in &*self.jar_files.borrow() {
                if jar_file.has_class(class_name) {
                    let read_data = jar_file.read_class(class_name)?;
                    let class_file = ClassFile::from_data(self.gc_ctx, read_data)?;
                    let class = Class::from_class_file(self, class_file)?;
                    self.register_class(class);

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
