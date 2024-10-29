use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::field::Field;
use super::method::Method;
use super::object::Object;
use super::value::{Value, ValueType};
use super::vtable::VTable;

use crate::classfile::class::ClassFile;
use crate::classfile::flags::{ClassFlags, FieldFlags, MethodFlags};
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::{Ref, RefCell};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Class(Gc<ClassData>);

struct ClassData {
    class_file: Option<ClassFile>,
    flags: ClassFlags,

    name: JvmString,

    super_class: Option<Class>,

    // If this class represents an array (T[]), the descriptor of the value type of the array.
    array_value_type: Option<Descriptor>,

    static_field_vtable: VTable<(JvmString, Descriptor)>,
    static_fields: Box<[Field]>,

    instance_field_vtable: VTable<(JvmString, Descriptor)>,
    // The values present on the class are the default values: when instantiating
    // an instance, the `instance_fields` should be cloned and added to the instance.
    instance_fields: Box<[Field]>,

    method_data: RefCell<Option<MethodData>>,
}

struct MethodData {
    static_method_vtable: VTable<(JvmString, MethodDescriptor)>,
    static_methods: Box<[Method]>,

    instance_method_vtable: VTable<(JvmString, MethodDescriptor)>,
    instance_methods: Box<[Method]>,
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Class").field("name", &self.name()).finish()
    }
}

impl Class {
    pub fn from_class_file(context: Context, class_file: ClassFile) -> Result<Self, Error> {
        let name = class_file.this_class();
        let super_class_name = class_file.super_class();

        let super_class = super_class_name
            .map(|name| context.lookup_class(name))
            .transpose()?;

        let fields = class_file.fields();

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

        let static_field_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_field_names);

        let instance_field_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            super_class.map(|c| c.instance_field_vtable()),
            instance_field_names,
        );

        let created_class = Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: Some(class_file),
                flags: class_file.flags(),
                name,
                super_class,

                array_value_type: None,

                static_field_vtable,
                static_fields: static_fields.into_boxed_slice(),
                instance_field_vtable,
                instance_fields: instance_fields.into_boxed_slice(),

                method_data: RefCell::new(None),
            },
        ));

        Ok(created_class)
    }

    // This must be called after the Class is registered.
    pub fn load_method_data(self, context: Context) -> Result<(), Error> {
        let class_file = self.class_file().unwrap();
        let super_class = self.super_class();

        let methods = class_file.methods();

        let mut static_method_names = Vec::with_capacity(methods.len());
        let mut static_methods = Vec::with_capacity(methods.len());
        let mut instance_method_names = Vec::with_capacity(methods.len());
        let mut instance_methods =
            super_class.map_or(Vec::new(), |c| c.instance_methods().to_vec());

        for method in methods {
            if method.flags().contains(MethodFlags::STATIC) {
                let created_method = Method::from_method(context, method, self)?;

                static_method_names.push((method.name(), created_method.descriptor()));
                static_methods.push(created_method);
            } else {
                let created_method = Method::from_method(context, method, self)?;

                instance_method_names.push((method.name(), created_method.descriptor()));
                instance_methods.push(created_method);
            }
        }

        let static_method_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, static_method_names);

        let instance_method_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            super_class.map(|c| *c.instance_method_vtable()),
            instance_method_names,
        );

        *self.0.method_data.borrow_mut() = Some(MethodData {
            static_method_vtable,
            static_methods: static_methods.into_boxed_slice(),
            instance_method_vtable,
            instance_methods: instance_methods.into_boxed_slice(),
        });

        // Now parse the actual ops, to ensure that the vtables have already been filled out
        for method in &*self.static_methods() {
            method.parse_info(context)?;
        }

        for method in &*self.instance_methods() {
            method.parse_info(context)?;
        }

        let clinit_string = context.common.clinit_name;
        let void_descriptor = context.common.noargs_void_desc;
        let clinit_method_idx = static_method_vtable.lookup((clinit_string, void_descriptor));
        if let Some(clinit_method_idx) = clinit_method_idx {
            // If this class actually has a clinit method, run it
            let clinit_method = self.static_methods()[clinit_method_idx];

            clinit_method.exec(context, &[])?;
        }

        Ok(())
    }

    pub fn create_object_class(context: Context) -> Self {
        let object_class_name = context.common.java_lang_object;
        let init_name = context.common.init_name;
        let void_descriptor = context.common.noargs_void_desc;

        let instance_method_names = vec![(init_name, void_descriptor)];

        let instance_methods = vec![Method::empty(
            context.gc_ctx,
            void_descriptor,
            MethodFlags::PUBLIC,
        )];

        let instance_method_vtable =
            VTable::from_parent_and_keys(context.gc_ctx, None, instance_method_names);

        let method_data = MethodData {
            static_method_vtable: VTable::empty(context.gc_ctx),
            static_methods: Box::new([]),
            instance_method_vtable,
            instance_methods: instance_methods.into_boxed_slice(),
        };

        Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: None,
                flags: ClassFlags::PUBLIC,
                name: object_class_name,
                super_class: None,

                array_value_type: None,

                static_field_vtable: VTable::empty(context.gc_ctx),
                static_fields: Box::new([]),
                instance_field_vtable: VTable::empty(context.gc_ctx),
                instance_fields: Box::new([]),

                method_data: RefCell::new(Some(method_data)),
            },
        ))
    }

    pub fn for_array(context: Context, array_descriptor: Descriptor) -> Self {
        let object_class_name = context.common.java_lang_object;
        let object_class = context
            .lookup_class(object_class_name)
            .expect("Object class should exist");

        let instance_methods = object_class.instance_methods();
        let instance_method_vtable = *object_class.instance_method_vtable();

        let method_data = MethodData {
            static_method_vtable: VTable::empty(context.gc_ctx),
            static_methods: Box::new([]),
            instance_method_vtable,
            instance_methods: instance_methods.clone(),
        };

        Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: None,
                flags: ClassFlags::PUBLIC,
                name: JvmString::new(context.gc_ctx, array_descriptor.to_string()),
                super_class: Some(object_class),

                array_value_type: Some(array_descriptor.array_inner_descriptor().unwrap()),

                static_field_vtable: VTable::empty(context.gc_ctx),
                static_fields: Box::new([]),
                instance_field_vtable: VTable::empty(context.gc_ctx),
                instance_fields: Box::new([]),

                method_data: RefCell::new(Some(method_data)),
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

    pub fn static_method_vtable(&self) -> Ref<VTable<(JvmString, MethodDescriptor)>> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().static_method_vtable
        })
    }

    pub fn static_methods(&self) -> Ref<Box<[Method]>> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().static_methods
        })
    }

    pub fn static_field_vtable(self) -> VTable<(JvmString, Descriptor)> {
        self.0.static_field_vtable
    }

    pub fn static_fields(&self) -> &[Field] {
        &self.0.static_fields
    }

    pub fn instance_method_vtable(&self) -> Ref<VTable<(JvmString, MethodDescriptor)>> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().instance_method_vtable
        })
    }

    pub fn instance_methods(&self) -> Ref<Box<[Method]>> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().instance_methods
        })
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

    pub fn matches_class(self, checked_class: Class) -> bool {
        if Gc::ptr_eq(self.0, checked_class.0) {
            true
        } else {
            self.has_super_class(checked_class)
        }
    }

    // This does not call the constructor.
    pub fn new_instance(self, gc_ctx: GcCtx) -> Object {
        // TODO can you somehow instantiate an array class?
        assert!(self.0.array_value_type.is_none());

        Object::from_class(gc_ctx, self)
    }

    pub fn call_static(
        self,
        context: Context,
        args: &[Value],
        name_and_descriptor: (JvmString, MethodDescriptor),
    ) -> Result<Option<Value>, Error> {
        let method_idx = self
            .static_method_vtable()
            .lookup(name_and_descriptor)
            .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

        let method = self.static_methods()[method_idx];

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
