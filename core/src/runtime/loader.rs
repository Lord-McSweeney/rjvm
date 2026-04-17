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

/// A representation of a Java class loader.
///
/// This implementation is not 100% correct, but it works for most use cases.
/// There are probably at least a few bugs with the way array classes are
/// registered.
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
            .field("object", &self.0.object)
            .finish()
    }
}

impl ClassLoader {
    pub(crate) fn bootstrap(gc_ctx: GcCtx, backend: Gc<Box<dyn LoaderBackend>>) -> Self {
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

    /// Create a new `ClassLoader` instance.
    pub fn with_parent(context: &Context, parent: ClassLoader, object: Object) -> Self {
        Self(Gc::new(
            context.gc_ctx,
            ClassLoaderData {
                parent: Some(parent),
                backend: context.loader_backend(),
                load_sources: RefCell::new(Vec::new()),
                class_registry: RefCell::new(HashMap::new()),
                array_classes: RefCell::new(HashMap::new()),
                object: Some(object),
            },
        ))
    }

    /// Get the parent loader of this `ClassLoader`, or `None` if it's the
    /// bootstrap loader.
    pub fn parent(self) -> Option<ClassLoader> {
        self.0.parent
    }

    /// Adds a [`ResourceLoadSource`] as one of this `ClassLoader`'s sources for
    /// loading data.
    pub fn add_source(self, source: ResourceLoadSource) {
        self.0.load_sources.borrow_mut().push(source);
    }

    /// Register a [`Class`] in this class loader's registry.
    ///
    /// This method will return an error if a class with the given class's name
    /// already exists in the registry.
    pub fn define_class(&self, context: &Context, class: Class) -> Result<(), Error> {
        let class_name = class.name();
        let mut registry = self.0.class_registry.borrow_mut();

        if !registry.contains_key(&class_name) {
            registry.insert(class_name, class);

            Ok(())
        } else {
            Err(context.linkage_error(&format!(
                "attempted duplicate class definition for {}",
                class_name
            )))
        }
    }

    /// Lookup a class using this `ClassLoader`. This will register the class
    /// in the correct `ClassLoader`'s registry. This will handle array classes
    /// correctly.
    ///
    /// This method will try to find the class on ancestor loaders if it's not
    /// found on this one.
    pub fn lookup_class(self, context: &Context, class_name: JvmString) -> Result<Class, Error> {
        match self.load_class(context, class_name) {
            Ok(Some(class)) => Ok(class),
            Ok(None) => Err(context.no_class_def_found_error(&*class_name)),
            Err(err) => Err(err),
        }
    }

    /// Find an already-loaded class on this `ClassLoader` (i.e. one that is
    /// in the registry already).
    pub fn find_loaded_class(self, class_name: JvmString) -> Option<Class> {
        let class_registry = self.0.class_registry.borrow();
        class_registry.get(&class_name).copied()
    }

    /// Like `lookup_class`, but returns `Ok(None)` when the class is not found.
    ///
    /// This method will try to find the class on ancestor loaders if it's not
    /// found on this one.
    pub fn load_class(
        self,
        context: &Context,
        class_name: JvmString,
    ) -> Result<Option<Class>, Error> {
        if let Some(class) = self.find_loaded_class(class_name) {
            return Ok(Some(class));
        }

        if let Some(parent) = self.parent() {
            if let Some(class) = parent.load_class(context, class_name)? {
                return Ok(Some(class));
            }
        }

        self.find_class(context, class_name)
    }

    /// Attempts to load a class on this class loader. This will not use this
    /// loader's class cache. This will not check the parent class loaders. If
    /// the class was not found on this loader, this will return `Ok(None)`. If
    /// the class was found on this loader, but attempting to parse the class
    /// resulted in an error, this will return `Err`.
    pub fn find_class(
        self,
        context: &Context,
        class_name: JvmString,
    ) -> Result<Option<Class>, Error> {
        if let Some(element_name) = class_name.strip_prefix('[') {
            // Special handling for array classes
            let element_name = JvmString::new(context.gc_ctx, element_name.to_string());
            let Some(element_descriptor) =
                Descriptor::try_from_string(context.gc_ctx, element_name)
            else {
                // Invalid descriptor
                return Ok(None);
            };

            let Some(resolved_descriptor) =
                ResolvedDescriptor::try_from_descriptor(context, self, element_descriptor)?
            else {
                // Failed to lookup a class in the descriptor
                return Ok(None);
            };

            let created_class = ClassLoader::array_class_for(context, resolved_descriptor);
            // `array_class_for` will register the class in the correct
            // `ClassLoader`'s registry

            Ok(Some(created_class))
        } else {
            // Not an array class, just try to load it as usual
            let full_name = class_name.to_string().clone() + ".class";
            let data = self.load_own_resource(&full_name);
            if let Some(data) = data {
                let class_file = ClassFile::from_data(context.gc_ctx, data)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let class = Class::from_class_file(context, self, class_file)?;

                self.define_class(context, class)?;

                Ok(Some(class))
            } else {
                Ok(None)
            }
        }
    }

    /// Create and register an array class for the given descriptor in the correct
    /// `ClassLoader`'s registry, or return an existing one. The `descriptor` is
    /// the inner class of the desired array.
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
            self.define_class(context, created_class)
                .expect("Array class didn't already exist");

            created_class
        }
    }

    /// Load a resource from this `ClassLoader`.
    ///
    /// This method will try to find the resource on ancestor loaders if it's
    /// not found on this one.
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

    /// Return the instance of `java.lang.ClassLoader` for this `ClassLoader`.
    /// This will return `None` if this `ClassLoader` is the bootstrap loader.
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

/// A trait that describes a type that can load external (i.e. filesystem)
/// resources. This "loader backend" is used for loading bootstrap and system
/// classes.
pub trait LoaderBackend {
    fn load_filesystem_resource(&self, resource_name: &str) -> Option<Vec<u8>>;
}

/// A place to search for an external resource.
pub enum ResourceLoadSource {
    /// This class was loaded directly from the filesystem. When searching
    /// for resources, look at the files in the directory of this class.
    FileSystem,

    /// This class was loaded from a JAR file. When searching for resources,
    /// look at the files in the directory of this class in the JAR.
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
