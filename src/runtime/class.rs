use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::field::Field;
use super::method::Method;
use super::value::Value;
use super::vtable::VTable;

use crate::classfile::class::ClassFile;
use crate::classfile::flags::{ClassFlags, FieldFlags, MethodFlags};
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub struct Class(Gc<ClassData>);

struct ClassData {
    class_file: Option<ClassFile>,
    flags: ClassFlags,

    name: JvmString,

    super_class: Option<Class>,

    static_method_vtable: VTable<(JvmString, MethodDescriptor)>,
    static_methods: Box<[Method]>,

    static_field_vtable: VTable<(JvmString, Descriptor)>,
    static_fields: Box<[Field]>,

    instance_method_vtable: VTable<(JvmString, MethodDescriptor)>,
    instance_methods: Box<[Method]>,

    instance_field_vtable: VTable<(JvmString, Descriptor)>,
    // The values present on the class are the default values: when instantiating
    // an instance, the `instance_fields` should be cloned and added to the instance.
    instance_fields: Box<[Field]>,
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
        let mut instance_field_names = Vec::with_capacity(fields.len());
        let mut instance_fields = super_class.map_or(Vec::new(), |c| c.instance_fields().to_vec());

        for field in fields {
            if field.flags().contains(FieldFlags::STATIC) {
                let created_field = Field::from_field(context.gc_ctx, field)?;

                static_field_names.push((field.name(), created_field.descriptor()));
                static_fields.push(created_field);
            } else {
                let created_field = Field::from_field(context.gc_ctx, field)?;

                instance_field_names.push((field.name(), created_field.descriptor()));
                instance_fields.push(created_field);
            }
        }

        let mut static_method_names = Vec::with_capacity(methods.len());
        let mut static_methods = Vec::with_capacity(methods.len());
        let mut instance_method_names = Vec::with_capacity(methods.len());
        let mut instance_methods =
            super_class.map_or(Vec::new(), |c| c.instance_methods().to_vec());

        for method in methods {
            if method.flags().contains(MethodFlags::STATIC) {
                let created_method = Method::from_method(context.gc_ctx, method)?;

                static_method_names.push((method.name(), created_method.descriptor()));
                static_methods.push(created_method);
            } else {
                let created_method = Method::from_method(context.gc_ctx, method)?;

                instance_method_names.push((method.name(), created_method.descriptor()));
                instance_methods.push(created_method);
            }
        }

        let static_field_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_field_names);

        let static_method_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_method_names);

        let instance_field_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            super_class.map(|c| c.instance_field_vtable()),
            instance_field_names,
        );

        let instance_method_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            super_class.map(|c| c.instance_method_vtable()),
            instance_method_names,
        );

        let created_class = Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: Some(class_file),
                flags: class_file.flags(),
                name,
                super_class,
                static_method_vtable,
                static_methods: static_methods.into_boxed_slice(),
                static_field_vtable,
                static_fields: static_fields.into_boxed_slice(),
                instance_method_vtable,
                instance_methods: instance_methods.into_boxed_slice(),
                instance_field_vtable,
                instance_fields: instance_fields.into_boxed_slice(),
            },
        ));

        for static_method in &created_class.0.static_methods {
            static_method.set_class_and_parse_code(context, created_class)?;
        }

        for instance_method in &created_class.0.instance_methods {
            instance_method.set_class_and_parse_code(context, created_class)?;
        }

        Ok(created_class)
    }

    pub fn create_object_class(gc_ctx: GcCtx) -> Self {
        let object_class_name = JvmString::new(gc_ctx, "java/lang/Object".to_string());
        let init_name = JvmString::new(gc_ctx, "<init>".to_string());
        let void_descriptor_name = JvmString::new(gc_ctx, "()V".to_string());

        let void_descriptor =
            MethodDescriptor::from_string(gc_ctx, void_descriptor_name).expect("Valid descriptor");

        let instance_method_names = vec![(init_name, void_descriptor)];

        let instance_methods = vec![Method::empty(gc_ctx, void_descriptor, MethodFlags::PUBLIC)];

        let instance_method_vtable =
            VTable::from_parent_and_keys(gc_ctx, None, instance_method_names);

        Self(Gc::new(
            gc_ctx,
            ClassData {
                class_file: None,
                flags: ClassFlags::PUBLIC,
                name: object_class_name,
                super_class: None,
                static_method_vtable: VTable::empty(gc_ctx),
                static_methods: Box::new([]),
                static_field_vtable: VTable::empty(gc_ctx),
                static_fields: Box::new([]),
                instance_method_vtable,
                instance_methods: instance_methods.into_boxed_slice(),
                instance_field_vtable: VTable::empty(gc_ctx),
                instance_fields: Box::new([]),
            },
        ))
    }

    pub fn class_file(&self) -> &Option<ClassFile> {
        &self.0.class_file
    }

    pub fn is_interface(self) -> bool {
        self.0.flags.contains(ClassFlags::INTERFACE)
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn super_class(self) -> Option<Class> {
        self.0.super_class
    }

    pub fn static_method_vtable(self) -> VTable<(JvmString, MethodDescriptor)> {
        self.0.static_method_vtable
    }

    pub fn static_methods(&self) -> &[Method] {
        &self.0.static_methods
    }

    pub fn static_field_vtable(self) -> VTable<(JvmString, Descriptor)> {
        self.0.static_field_vtable
    }

    pub fn static_fields(&self) -> &[Field] {
        &self.0.static_fields
    }

    pub fn instance_method_vtable(self) -> VTable<(JvmString, MethodDescriptor)> {
        self.0.instance_method_vtable
    }

    pub fn instance_methods(&self) -> &[Method] {
        &self.0.instance_methods
    }

    pub fn instance_field_vtable(self) -> VTable<(JvmString, Descriptor)> {
        self.0.instance_field_vtable
    }

    pub fn instance_fields(&self) -> &[Field] {
        &self.0.instance_fields
    }

    // This does not check if the checked class is an interface implemented by this class.
    pub fn has_super_class(self, checked_class: Class) -> bool {
        let mut current_class = self.super_class();
        while let Some(some_class) = current_class {
            if Gc::ptr_eq(some_class.0, checked_class.0) {
                return true;
            }

            current_class = some_class.super_class();
        }

        return false;
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
