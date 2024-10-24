use super::context::Context;
use super::error::Error;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub struct Class(Gc<ClassData>);

struct ClassData {
    class_file: Option<ClassFile>,

    name: JvmString,

    super_class: Option<Class>,
}

impl Class {
    pub fn from_class_file(context: Context, class_file: ClassFile) -> Result<Self, Error> {
        let name = class_file.this_class();
        let super_class_name = class_file.super_class();

        let super_class = super_class_name
            .map(|name| context.lookup_class(name))
            .transpose()?;

        Ok(Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: Some(class_file),
                name,
                super_class,
            },
        )))
    }

    pub fn new(gc_ctx: GcCtx, name: JvmString) -> Self {
        Self(Gc::new(
            gc_ctx,
            ClassData {
                class_file: None,
                name,
                super_class: None,
            },
        ))
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }
}

impl Trace for Class {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for ClassData {
    fn trace(&self) {
        self.class_file.trace();
        self.name.trace();
        self.super_class.trace();
    }
}
