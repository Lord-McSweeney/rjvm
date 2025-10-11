use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::field::{Field, FieldRef};
use super::loader::ClassLoader;
use super::method::Method;
use super::object::Object;
use super::value::Value;
use super::vtable::{InstanceMethodVTable, VTable};

use crate::classfile::class::ClassFile;
use crate::classfile::flags::{ClassFlags, FieldFlags, MethodFlags};
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::{Cell, OnceCell, Ref, RefCell};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Class(Gc<ClassData>);

struct ClassData {
    class_file: Option<ClassFile>,

    loader: Option<ClassLoader>,

    flags: ClassFlags,

    name: JvmString,

    super_class: Option<Class>,

    // The `java.lang.Class` object for this `Class`, lazily initialized
    object: OnceCell<Object>,

    // The interfaces that this class directly implements.
    own_interfaces: Box<[Class]>,

    // All the interfaces that this class implements, including those implemented
    // by subclasses.
    all_interfaces: Box<[Class]>,

    // If this class represents an array (T[]), the descriptor of the value type of the array.
    array_value_type: Option<ResolvedDescriptor>,

    // The primitive type that this class represents.
    primitive_type: Option<PrimitiveType>,

    static_field_vtable: VTable<Descriptor>,
    static_fields: Box<[FieldRef]>,

    instance_field_vtable: VTable<Descriptor>,
    // The values present on the class are the default values: when instantiating
    // an instance, the `instance_fields` should be cloned and added to the instance.
    instance_fields: Box<[Field]>,

    method_data: RefCell<Option<MethodData>>,

    clinit_method: Cell<Option<Method>>,
    clinit_run: Cell<bool>,
}

struct MethodData {
    static_method_vtable: VTable<MethodDescriptor>,
    static_methods: Box<[Method]>,

    instance_method_vtable: InstanceMethodVTable,
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[class {}]", self.name())
    }
}

impl Class {
    pub fn from_class_file(
        context: &Context,
        loader: ClassLoader,
        class_file: ClassFile,
    ) -> Result<Self, Error> {
        let name = class_file.this_class();
        let super_class_name = class_file.super_class();

        let super_class = super_class_name
            .map(|name| loader.lookup_class(context, name))
            .transpose()?;

        let mut own_interfaces = Vec::new();
        for interface in class_file.interfaces() {
            // Hopefully we won't get a class that tries to implement itself as an interface
            let class = loader.lookup_class(context, *interface)?;
            if !class.is_interface() {
                return Err(Error::Native(NativeError::ClassNotInterface));
            }

            own_interfaces.push(class);
        }

        let mut all_interfaces = HashSet::new();
        let mut class_queue: Vec<Class> = Vec::with_capacity(own_interfaces.len() + 2);

        for interface in &own_interfaces {
            class_queue.push(*interface);
            all_interfaces.insert(*interface);
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
        let mut static_fields = super_class.map_or(Vec::new(), |c| c.static_fields().to_vec());
        let mut instance_field_names = Vec::with_capacity(fields.len());
        let mut instance_fields = super_class.map_or(Vec::new(), |c| c.instance_fields().to_vec());

        for interface in &all_interfaces {
            let interface_statics = interface.static_fields();

            for field in interface_statics {
                static_field_names.push((field.name(), field.descriptor()));
                static_fields.push(*field);
            }
        }

        for field in fields {
            if field.flags().contains(FieldFlags::STATIC) {
                let created_field = FieldRef::from_field(context, class_file, field)?;

                static_field_names.push((field.name(), created_field.descriptor()));
                static_fields.push(created_field);
            } else {
                let created_field = Field::from_field(context, class_file, field)?;

                instance_field_names.push((field.name(), created_field.descriptor()));
                instance_fields.push(created_field);
            }
        }

        let static_field_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            None,
            super_class.map(|c| c.static_field_vtable()),
            static_field_names,
        );

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
                loader: Some(loader),

                flags: class_file.flags(),

                name,
                super_class,

                object: OnceCell::new(),

                own_interfaces: own_interfaces.into_boxed_slice(),
                all_interfaces: all_interfaces.iter().copied().collect::<Box<[_]>>(),

                array_value_type: None,
                primitive_type: None,

                static_field_vtable,
                static_fields: static_fields.into_boxed_slice(),
                instance_field_vtable,
                instance_fields: instance_fields.into_boxed_slice(),

                method_data: RefCell::new(None),
                clinit_method: Cell::new(None),
                clinit_run: Cell::new(false),
            },
        ));

        Ok(created_class)
    }

    // This must be called after the Class is registered.
    pub fn load_methods(self, context: &Context) -> Result<(), Error> {
        let class_file = self.class_file().unwrap();
        let super_class = self.super_class();

        let methods = class_file.methods();

        let mut static_method_names = Vec::with_capacity(methods.len());
        let mut static_methods = super_class.map_or(Vec::new(), |c| c.static_methods().to_vec());

        // Instance methods are special because we need dynamic dispatch for them
        let mut instance_methods = Vec::with_capacity(methods.len());

        for method in methods {
            if method.flags().contains(MethodFlags::STATIC) {
                let created_method = Method::from_method(context, method, self)?;

                static_method_names.push((method.name(), created_method.descriptor()));
                static_methods.push(created_method);
            } else {
                let created_method = Method::from_method(context, method, self)?;

                let key = (method.name(), created_method.descriptor());

                instance_methods.push((key, created_method));
            }
        }

        let static_method_vtable = VTable::from_parent_and_keys(
            context.gc_ctx,
            Some(self),
            super_class.map(|c| *c.static_method_vtable()),
            static_method_names,
        );

        let instance_method_vtable = InstanceMethodVTable::from_parent_and_keys(
            context.gc_ctx,
            Some(self),
            super_class.map(|c| *c.instance_method_vtable()),
            instance_methods,
        );

        *self.0.method_data.borrow_mut() = Some(MethodData {
            static_method_vtable,
            static_methods: static_methods.into_boxed_slice(),
            instance_method_vtable,
        });

        let clinit_string = context.common.clinit_name;
        let void_descriptor = context.common.noargs_void_desc;
        let clinit_method_idx = static_method_vtable.lookup((clinit_string, void_descriptor));

        // If this class actually has a clinit method, queue it
        // (don't run it now as it could potentially trigger a GC, and we
        // may have Gc pointers stored only on the stack at the moment)
        self.0
            .clinit_method
            .set(clinit_method_idx.map(|i| self.static_methods()[i]));

        Ok(())
    }

    /// DO NOT USE THIS TO GET ARRAY CLASSES! It WILL create duplicate classes
    /// for the same `array_type`! Use `ClassLoader::array_class_for` instead.
    pub fn for_array(context: &Context, array_type: ResolvedDescriptor) -> Self {
        let object_class = context.object_class();

        let instance_method_vtable = *object_class.instance_method_vtable();

        let method_data = MethodData {
            static_method_vtable: VTable::empty(context.gc_ctx),
            static_methods: Box::new([]),
            // FIXME is this correct?
            instance_method_vtable,
        };

        let mut name = String::with_capacity(8);
        name.push('[');
        name.push_str(&array_type.to_string());

        Self(Gc::new(
            context.gc_ctx,
            ClassData {
                class_file: None,
                // Array classes have the loader of their inner class
                loader: array_type.class().and_then(|c| c.loader()),

                flags: ClassFlags::PUBLIC | ClassFlags::FINAL,

                name: JvmString::new(context.gc_ctx, name),
                super_class: Some(object_class),

                object: OnceCell::new(),

                own_interfaces: Box::new([]),
                all_interfaces: Box::new([]),

                array_value_type: Some(array_type),
                primitive_type: None,

                static_field_vtable: VTable::empty(context.gc_ctx),
                static_fields: Box::new([]),
                instance_field_vtable: VTable::empty(context.gc_ctx),
                instance_fields: Box::new([]),

                method_data: RefCell::new(Some(method_data)),

                clinit_method: Cell::new(None),
                clinit_run: Cell::new(true),
            },
        ))
    }

    // Creates a builtin class for one of the primitive types.
    pub fn for_primitive(gc_ctx: GcCtx, primitive_type: PrimitiveType) -> Self {
        let method_data = MethodData {
            static_method_vtable: VTable::empty(gc_ctx),
            static_methods: Box::new([]),
            // FIXME do we need to add any methods to this vtable?
            instance_method_vtable: InstanceMethodVTable::empty(gc_ctx),
        };

        Self(Gc::new(
            gc_ctx,
            ClassData {
                class_file: None,
                loader: None,

                flags: ClassFlags::PUBLIC | ClassFlags::FINAL,

                name: JvmString::new(gc_ctx, primitive_type.name().to_string()),
                super_class: None,

                object: OnceCell::new(),

                own_interfaces: Box::new([]),
                all_interfaces: Box::new([]),

                array_value_type: None,
                primitive_type: Some(primitive_type),

                static_field_vtable: VTable::empty(gc_ctx),
                static_fields: Box::new([]),
                instance_field_vtable: VTable::empty(gc_ctx),
                instance_fields: Box::new([]),

                method_data: RefCell::new(Some(method_data)),

                clinit_method: Cell::new(None),
                clinit_run: Cell::new(true),
            },
        ))
    }

    pub fn class_file(&self) -> &Option<ClassFile> {
        &self.0.class_file
    }

    pub fn is_abstract(self) -> bool {
        self.0.flags.contains(ClassFlags::ABSTRACT)
    }

    pub fn is_interface(self) -> bool {
        self.0.flags.contains(ClassFlags::INTERFACE)
    }

    pub fn is_primitive(self) -> bool {
        self.primitive_type().is_some()
    }

    pub fn primitive_type(self) -> Option<PrimitiveType> {
        self.0.primitive_type
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn loader(self) -> Option<ClassLoader> {
        self.0.loader
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

    pub fn static_method_vtable(&self) -> Ref<VTable<MethodDescriptor>> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().static_method_vtable
        })
    }

    pub fn static_methods(&self) -> Ref<Box<[Method]>> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().static_methods
        })
    }

    pub fn static_field_vtable(self) -> VTable<Descriptor> {
        self.0.static_field_vtable
    }

    pub fn static_fields(&self) -> &[FieldRef] {
        &self.0.static_fields
    }

    pub fn instance_method_vtable(&self) -> Ref<InstanceMethodVTable> {
        Ref::map(self.0.method_data.borrow(), |data| {
            &data.as_ref().unwrap().instance_method_vtable
        })
    }

    pub fn instance_field_vtable(self) -> VTable<Descriptor> {
        self.0.instance_field_vtable
    }

    pub fn instance_fields(&self) -> &[Field] {
        &self.0.instance_fields
    }

    // Return an instance of `java.lang.Class` for this `Class`.
    pub fn get_or_init_object(self, context: &Context) -> Object {
        *self.0.object.get_or_init(|| {
            let id = context.add_class_object(self);

            let object = Object::class_object(context);
            object.set_field(0, Value::Integer(id));

            object
        })
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

    /// Implements the JVM `checkcast` instruction, returning true if the cast
    /// was successful and false if an error should be thrown.
    pub fn check_cast(self, checked_class: Class) -> bool {
        if let (Some(our_inner), Some(other_inner)) =
            (self.array_value_type(), checked_class.array_value_type())
        {
            if let (Some(our_inner), Some(other_inner)) = (our_inner.class(), other_inner.class()) {
                // Recursively look into the next inner type
                return our_inner.check_cast(other_inner);
            } else {
                // >=1 of the descriptors is primitive, just check descriptor equality
                return our_inner == other_inner;
            }
        }

        self.matches_class(checked_class) || self.implements_interface(checked_class)
    }

    // This does not call the constructor.
    pub fn new_instance(self, gc_ctx: GcCtx) -> Object {
        // TODO can you somehow instantiate an array class?
        assert!(self.0.array_value_type.is_none());

        Object::from_class(gc_ctx, self)
    }

    pub fn run_clinit(self, context: &Context) -> Result<(), Error> {
        if !self.0.clinit_run.get() {
            self.0.clinit_run.set(true);

            if let Some(clinit) = self.0.clinit_method.get() {
                clinit.exec(context, &[])?;
            }

            if let Some(super_class) = self.super_class() {
                super_class.run_clinit(context)?;
            }

            for interface in self.own_interfaces() {
                interface.run_clinit(context)?;
            }
        }

        Ok(())
    }

    pub fn as_ptr(self) -> *const () {
        Gc::as_ptr(self.0) as *const ()
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
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for ClassData {
    #[inline]
    fn trace(&self) {
        self.class_file.trace();
        self.loader.trace();
        self.name.trace();
        self.super_class.trace();
        self.object.trace();
        self.own_interfaces.trace();
        self.all_interfaces.trace();
        self.array_value_type.trace();

        self.static_field_vtable.trace();
        self.static_fields.trace();
        self.instance_field_vtable.trace();
        self.instance_fields.trace();

        if let Some(method_data) = &*self.method_data.borrow() {
            method_data.static_method_vtable.trace();
            method_data.static_methods.trace();

            method_data.instance_method_vtable.trace();
        }

        self.clinit_method.trace();
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Void,
}

impl PrimitiveType {
    fn name(self) -> &'static str {
        match self {
            PrimitiveType::Boolean => "boolean",
            PrimitiveType::Byte => "byte",
            PrimitiveType::Char => "char",
            PrimitiveType::Short => "short",
            PrimitiveType::Int => "int",
            PrimitiveType::Long => "long",
            PrimitiveType::Float => "float",
            PrimitiveType::Double => "double",
            PrimitiveType::Void => "void",
        }
    }

    pub fn get_all() -> Vec<PrimitiveType> {
        vec![
            PrimitiveType::Boolean,
            PrimitiveType::Byte,
            PrimitiveType::Char,
            PrimitiveType::Short,
            PrimitiveType::Int,
            PrimitiveType::Long,
            PrimitiveType::Float,
            PrimitiveType::Double,
            PrimitiveType::Void,
        ]
    }
}

impl Trace for PrimitiveType {
    fn trace(&self) {}
}
