use super::context::{Context, ResourceLoadType};
use super::descriptor::{Descriptor, MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::field::Field;
use super::method::Method;
use super::object::Object;
use super::value::Value;
use super::vtable::VTable;

use crate::classfile::class::ClassFile;
use crate::classfile::flags::{ClassFlags, FieldFlags, MethodFlags};
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Class(Gc<ClassData>);

struct ClassData {
    class_file: Option<ClassFile>,
    load_source: Option<ResourceLoadType>,

    flags: ClassFlags,

    name: JvmString,

    super_class: Option<Class>,

    // The interfaces that this class directly implements.
    own_interfaces: Box<[Class]>,

    // All the interfaces that this class implements, including those implemented
    // by subclasses.
    all_interfaces: Box<[Class]>,

    // If this class represents an array (T[]), the descriptor of the value type of the array.
    array_value_type: Option<ResolvedDescriptor>,

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
    pub fn from_class_file(
        context: Context,
        load_source: ResourceLoadType,
        class_file: ClassFile,
    ) -> Result<Self, Error> {
        let name = class_file.this_class();
        let super_class_name = class_file.super_class();

        let super_class = super_class_name
            .map(|name| context.lookup_class(name))
            .transpose()?;

        let mut own_interfaces = Vec::new();
        for interface in class_file.interfaces() {
            // Hopefully we won't get a class that tries to implement itself as an interface
            let class = context.lookup_class(*interface)?;
            if !class.is_interface() {
                return Err(Error::Native(NativeError::ClassNotInterface));
            }

            own_interfaces.push(class);
        }

        let mut all_interfaces = HashSet::new();
        let mut class_queue: Vec<Class> = Vec::with_capacity(own_interfaces.len() + 2);

        for interface in &own_interfaces {
            class_queue.push(*interface);
        }
        if let Some(super_class) = super_class {
            class_queue.push(super_class);
        }

        while let Some(class) = class_queue.pop() {
            for interface in class.own_interfaces() {
                if all_interfaces.insert(*interface) {
                    class_queue.push(*interface);
                }
            }

            if let Some(super_class) = class.super_class() {
                class_queue.push(super_class);
            }
        }

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
            VTable::from_parent_and_keys(context.gc_ctx, None, None, static_field_names);

        let instance_field_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            None,
            super_class.map(|c| c.instance_field_vtable()),
            instance_field_names,
        );

        let created_class = Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: Some(class_file),
                load_source: Some(load_source),

                flags: class_file.flags(),

                name,
                super_class,

                own_interfaces: own_interfaces.into_boxed_slice(),
                all_interfaces: all_interfaces.iter().copied().collect::<Box<[_]>>(),

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
    pub fn load_methods(self, context: Context) -> Result<(), Error> {
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
            VTable::from_parent_and_keys(context.gc_ctx, Some(self), None, static_method_names);

        let instance_method_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            Some(self),
            super_class.map(|c| *c.instance_method_vtable()),
            instance_method_names,
        );

        *self.0.method_data.borrow_mut() = Some(MethodData {
            static_method_vtable,
            static_methods: static_methods.into_boxed_slice(),
            instance_method_vtable,
            instance_methods: instance_methods.into_boxed_slice(),
        });

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

    pub fn for_array(context: Context, array_type: ResolvedDescriptor) -> Self {
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

        let mut name = String::with_capacity(8);
        name.push('[');
        name.push_str(&array_type.to_string());

        Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: None,
                load_source: None,

                flags: ClassFlags::PUBLIC,

                name: JvmString::new(context.gc_ctx, name),
                super_class: Some(object_class),

                own_interfaces: Box::new([]),
                all_interfaces: Box::new([]),

                array_value_type: Some(array_type),

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

    pub fn dot_name(self) -> String {
        self.0.name.replace('/', ".")
    }

    pub fn super_class(self) -> Option<Class> {
        self.0.super_class
    }

    pub fn own_interfaces(&self) -> &[Class] {
        &self.0.own_interfaces
    }

    pub fn array_value_type(self) -> Option<ResolvedDescriptor> {
        self.0.array_value_type
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

    pub fn implements_interface(self, checked_interface: Class) -> bool {
        self.0
            .all_interfaces
            .iter()
            .any(|i| *i == checked_interface)
    }

    pub fn matches_descriptor(self, descriptor: ResolvedDescriptor) -> bool {
        match descriptor {
            ResolvedDescriptor::Class(class) => self.matches_class(class),
            ResolvedDescriptor::Array(array_class) => array_class == self,
            _ => unreachable!(),
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

    pub fn load_resource(self, context: Context, resource_name: &String) -> Option<Vec<u8>> {
        self.0.load_source.as_ref().and_then(|load_source| {
            context.load_resource(load_source, self.name().to_string(), resource_name)
        })
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        Gc::as_ptr(self.0) == Gc::as_ptr(other.0)
    }
}

impl Eq for Class {}

impl Hash for Class {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Gc::as_ptr(self.0).hash(state);
    }
}

impl Trace for Class {
    fn trace(&self) {
        self.0.trace();

        self.static_method_vtable().trace();
        self.static_methods().trace();

        self.instance_method_vtable().trace();
        self.instance_methods().trace();
    }
}

impl Trace for ClassData {
    fn trace(&self) {
        self.class_file.trace();
        self.name.trace();
        self.super_class.trace();
        self.own_interfaces.trace();
        self.all_interfaces.trace();
        self.array_value_type.trace();

        self.static_field_vtable.trace();
        self.static_fields.trace();
        self.instance_field_vtable.trace();
        self.instance_fields.trace();
    }
}
