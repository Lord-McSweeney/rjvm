// Loader trait
use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, ResolvedDescriptor};
use super::error::Error;
use super::object::Object;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::string::JvmString;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt;
use hashbrown::HashMap;

#[derive(Clone, Copy)]
pub struct ClassLoader(Gc<ClassLoaderData>);

struct ClassLoaderData {
    parent: Option<ClassLoader>,

    backend: Gc<Box<dyn LoaderBackend>>,
    load_sources: RefCell<Vec<ResourceLoadSource>>,

    class_registry: RefCell<HashMap<JvmString, Class>>,
    array_classes: RefCell<HashMap<ResolvedDescriptor, Class>>,

    // The `java.lang.ClassLoader` object for this `ClassLoader`
    object: Option<Object>,
}

impl fmt::Debug for ClassLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ClassLoader")
            .field("parent", &self.parent())
            .finish()
    }
}

impl ClassLoader {
    pub fn bootstrap(gc_ctx: GcCtx, backend: Gc<Box<dyn LoaderBackend>>) -> Self {
        Self(Gc::new(
            gc_ctx,
            ClassLoaderData {
                parent: None,
                backend,
                load_sources: RefCell::new(Vec::new()),
                class_registry: RefCell::new(HashMap::new()),
                array_classes: RefCell::new(HashMap::new()),
                object: None,
            },
        ))
    }

    pub fn with_parent(
        gc_ctx: GcCtx,
        parent: ClassLoader,
        object: Object,
        backend: Gc<Box<dyn LoaderBackend>>,
    ) -> Self {
        Self(Gc::new(
            gc_ctx,
            ClassLoaderData {
                parent: Some(parent),
                backend,
                load_sources: RefCell::new(Vec::new()),
                class_registry: RefCell::new(HashMap::new()),
                array_classes: RefCell::new(HashMap::new()),
                object: Some(object),
            },
        ))
    }

    pub fn parent(self) -> Option<ClassLoader> {
        self.0.parent
    }

    pub fn add_source(self, source: ResourceLoadSource) {
        self.0.load_sources.borrow_mut().push(source);
    }

    fn register_class(&self, class: Class) {
        let class_name = class.name();
        let mut registry = self.0.class_registry.borrow_mut();

        if registry.contains_key(&class_name) {
            panic!("Attempted to register class {} twice", class_name);
        } else {
            registry.insert(class_name, class);
        }
    }

    // Lookup a class using this `ClassLoader`. This will register the class
    // in the correct `ClassLoader`'s registry. This will handle array classes
    // correctly.
    pub fn lookup_class(self, context: &Context, class_name: JvmString) -> Result<Class, Error> {
        match self.find_class(context, class_name) {
            Ok(Some(class)) => Ok(class),
            Ok(None) => Err(context.no_class_def_found_error(&*class_name)),
            Err(err) => Err(err),
        }
    }

    // Like `lookup_class`, but returns `Ok(None)` when the class is not found
    pub fn find_class(
        self,
        context: &Context,
        class_name: JvmString,
    ) -> Result<Option<Class>, Error> {
        if let Some(element_name) = class_name.strip_prefix('[') {
            // Special handling for array classes
            let element_name = JvmString::new(context.gc_ctx, element_name.to_string());
            let element_descriptor = Descriptor::try_from_string(context.gc_ctx, element_name)
                .ok_or_else(|| context.no_class_def_found_error(&*class_name))?;

            let resolved_descriptor =
                ResolvedDescriptor::from_descriptor(context, self, element_descriptor)?;

            let created_class = ClassLoader::array_class_for(context, resolved_descriptor);
            // `array_class_for` will register the class in the correct
            // `ClassLoader`'s registry

            Ok(Some(created_class))
        } else {
            // Not an array class, just recursively lookup on self and ancestors
            let mut current = Some(self);
            while let Some(current_loader) = current {
                let result = current_loader.lookup_own_class(context, class_name)?;
                if let Some(result) = result {
                    return Ok(Some(result));
                }

                current = current_loader.parent();
            }

            Ok(None)
        }
    }

    // Attempts to lookup a class on this class loader. This will not check the
    // parent class loaders. If the class was not found on this loader, this
    // will return `Ok(None)`. If the class was found on this loader, but
    // attempting to parse the class resulted in an error, this will return
    // `Err`.
    fn lookup_own_class(
        self,
        context: &Context,
        class_name: JvmString,
    ) -> Result<Option<Class>, Error> {
        let class_registry = self.0.class_registry.borrow();
        if let Some(class) = class_registry.get(&class_name) {
            return Ok(Some(*class));
        }
        drop(class_registry);

        // Couldn't find it in our registry, now try to load it
        let full_name = class_name.to_string().clone() + ".class";
        let data = self.load_own_resource(&full_name);
        if let Some(data) = data {
            let class_file = ClassFile::from_data(context.gc_ctx, data)
                .map_err(|e| Error::from_class_file_error(context, e))?;

            let class = Class::from_class_file(&context, self, class_file)?;

            self.register_class(class);

            class.load_methods(context)?;

            Ok(Some(class))
        } else {
            Ok(None)
        }
    }

    // Register an array class for the given descriptor in the correct
    // `ClassLoader`'s registry.
    pub fn array_class_for(context: &Context, descriptor: ResolvedDescriptor) -> Class {
        let correct_loader = descriptor
            .class()
            .and_then(|c| c.loader())
            .unwrap_or(context.bootstrap_loader());

        correct_loader.get_or_init_array_class(context, descriptor)
    }

    // Register an array class for the given descriptor in *this*
    // `ClassLoader`'s registry.
    fn get_or_init_array_class(self, context: &Context, descriptor: ResolvedDescriptor) -> Class {
        let array_classes = self.0.array_classes.borrow();

        if let Some(class) = array_classes.get(&descriptor) {
            *class
        } else {
            drop(array_classes);
            let created_class = Class::for_array(context, descriptor);
            self.0
                .array_classes
                .borrow_mut()
                .insert(descriptor, created_class);
            self.register_class(created_class);
            created_class
        }
    }

    pub fn load_resource(self, resource_name: &str) -> Option<Vec<u8>> {
        // Recursively lookup on self and ancestors
        let mut current = Some(self);
        while let Some(current_loader) = current {
            let result = current_loader.load_own_resource(resource_name);
            if let Some(result) = result {
                return Some(result);
            }

            current = current_loader.parent();
        }

        None
    }

    fn load_own_resource(self, resource_name: &str) -> Option<Vec<u8>> {
        let sources = self.0.load_sources.borrow();
        for source in &*sources {
            let result = source.load(&**self.0.backend, resource_name);

            if let Some(result) = result {
                return Some(result);
            }
        }

        None
    }

    // Return an instance of `java.lang.ClassLoader` for this `ClassLoader`.
    // This will return `None` if this `ClassLoader` is the bootstrap loader.
    pub fn object(self) -> Option<Object> {
        self.0.object
    }
}

impl PartialEq for ClassLoader {
    fn eq(&self, other: &Self) -> bool {
        Gc::as_ptr(self.0) == Gc::as_ptr(other.0)
    }
}

impl Eq for ClassLoader {}

impl Trace for ClassLoader {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for ClassLoaderData {
    fn trace(&self) {
        self.parent.trace();
        self.backend.trace_self();
        self.load_sources.trace();
        self.class_registry.trace();
        self.array_classes.trace();
        self.object.trace();
    }
}

pub trait LoaderBackend {
    fn load_filesystem_resource(&self, resource_name: &str) -> Option<Vec<u8>>;
}

pub enum ResourceLoadSource {
    // This class was loaded directly from the filesystem. When searching
    // for resources, look at the files in the directory of this class.
    FileSystem,

    // This class was loaded from a JAR file. When searching for resources,
    // look at the files in the directory of this class in the JAR.
    Jar(Jar),
}

impl ResourceLoadSource {
    fn load(&self, backend: &dyn LoaderBackend, resource_name: &str) -> Option<Vec<u8>> {
        match self {
            ResourceLoadSource::FileSystem => backend.load_filesystem_resource(resource_name),
            ResourceLoadSource::Jar(jar) => {
                let resource_name = resource_name.to_string();
                if jar.has_file(&resource_name) {
                    jar.read_file(resource_name).ok()
                } else {
                    None
                }
            }
        }
    }
}

impl Trace for ResourceLoadSource {
    fn trace(&self) {
        match self {
            ResourceLoadSource::Jar(jar) => jar.trace(),
            _ => {}
        }
    }
}
