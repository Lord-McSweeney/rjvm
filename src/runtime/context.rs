use super::class::Class;
use super::error::{Error, NativeError};

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Context {
    class_registry: Gc<RefCell<HashMap<JvmString, Class>>>,

    pub gc_ctx: GcCtx,
}

impl Context {
    pub fn new(gc_ctx: GcCtx) -> Self {
        let created_self = Self {
            class_registry: Gc::new(gc_ctx, RefCell::new(HashMap::new())),
            gc_ctx,
        };

        created_self.init_object_class();

        created_self
    }

    fn init_object_class(self) {
        let object_class_name = JvmString::new(self.gc_ctx, "java/lang/Object".to_string());
        let object_class = Class::new(self.gc_ctx, object_class_name);

        self.register_class(object_class);
    }

    pub fn lookup_class(self, class_name: JvmString) -> Result<Class, Error> {
        if let Some(class) = self.class_registry.borrow().get(&class_name) {
            Ok(*class)
        } else {
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
}

impl Trace for Context {
    fn trace(&self) {
        self.class_registry.trace();
        self.class_registry.borrow().trace();
    }
}
