use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::method::Method;
use super::vtable::VTable;

use crate::classfile::class::ClassFile;
use crate::classfile::flags::MethodFlags;
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub struct Class(Gc<ClassData>);

struct ClassData {
    class_file: Option<ClassFile>,

    name: JvmString,

    super_class: Option<Class>,

    static_method_vtable: VTable<(JvmString, MethodDescriptor)>,
    static_methods: Box<[Method]>,
}

impl Class {
    pub fn from_class_file(context: Context, class_file: ClassFile) -> Result<Self, Error> {
        let name = class_file.this_class();
        let super_class_name = class_file.super_class();

        let super_class = super_class_name
            .map(|name| context.lookup_class(name))
            .transpose()?;

        let methods = class_file.methods();

        let mut static_method_names = Vec::with_capacity(methods.len());
        let mut static_methods = Vec::with_capacity(methods.len());
        for method in methods {
            if method.flags().contains(MethodFlags::STATIC) {
                let created_method = Method::from_method(context.gc_ctx, method)?;

                static_method_names.push((method.name(), created_method.descriptor()));
                static_methods.push(created_method);
            }
        }

        let static_method_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_method_names);

        Ok(Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: Some(class_file),
                name,
                super_class,
                static_method_vtable,
                static_methods: static_methods.into_boxed_slice(),
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
                static_method_vtable: VTable::empty(gc_ctx),
                static_methods: Box::new([]),
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
