use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::field::Field;
use super::method::Method;
use super::value::Value;
use super::vtable::VTable;

use crate::classfile::class::ClassFile;
use crate::classfile::flags::{FieldFlags, MethodFlags};
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

    static_field_vtable: VTable<(JvmString, Descriptor)>,
    static_fields: Box<[Field]>,
}

impl Class {
    pub fn from_class_file(context: Context, class_file: ClassFile) -> Result<Self, Error> {
        let name = class_file.this_class();
        let super_class_name = class_file.super_class();

        let super_class = super_class_name
            .map(|name| context.lookup_class(name))
            .transpose()?;

        let fields = class_file.fields();
        let methods = class_file.methods();

        let mut static_field_names = Vec::with_capacity(fields.len());
        let mut static_fields = Vec::with_capacity(fields.len());
        for field in fields {
            if field.flags().contains(FieldFlags::STATIC) {
                let created_field = Field::from_field(context.gc_ctx, field)?;

                static_field_names.push((field.name(), created_field.descriptor()));
                static_fields.push(created_field);
            }
        }

        let mut static_method_names = Vec::with_capacity(methods.len());
        let mut static_methods = Vec::with_capacity(methods.len());
        for method in methods {
            if method.flags().contains(MethodFlags::STATIC) {
                let created_method = Method::from_method(context.gc_ctx, method)?;

                static_method_names.push((method.name(), created_method.descriptor()));
                static_methods.push(created_method);
            }
        }

        let static_field_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_field_names);

        let static_method_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_method_names);

        let created_class = Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: Some(class_file),
                name,
                super_class,
                static_method_vtable,
                static_methods: static_methods.into_boxed_slice(),
                static_field_vtable,
                static_fields: static_fields.into_boxed_slice(),
            },
        ));

        for static_method in &created_class.0.static_methods {
            static_method.set_class_and_parse_code(context, created_class)?;
        }

        Ok(created_class)
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
                static_field_vtable: VTable::empty(gc_ctx),
                static_fields: Box::new([]),
            },
        ))
    }

    pub fn class_file(&self) -> &Option<ClassFile> {
        &self.0.class_file
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn static_method_vtable(self) -> VTable<(JvmString, MethodDescriptor)> {
        self.0.static_method_vtable
    }

    pub fn static_field_vtable(self) -> VTable<(JvmString, Descriptor)> {
        self.0.static_field_vtable
    }

    pub fn call_static(
        self,
        context: Context,
        args: &[Value],
        name_and_descriptor: (JvmString, MethodDescriptor),
    ) -> Result<Option<Value>, Error> {
        let method_idx = self
            .0
            .static_method_vtable
            .lookup(name_and_descriptor)
            .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

        let method = self.0.static_methods[method_idx];

        method.exec(context, args)
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
