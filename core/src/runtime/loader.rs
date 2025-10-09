// Loader trait
use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, ResolvedDescriptor};
use super::error::Error;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::string::JvmString;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy)]
pub struct ClassLoader(Gc<ClassLoaderData>);

struct ClassLoaderData {
    parent: Option<ClassLoader>,

    backend: Gc<Box<dyn LoaderBackend>>,
    load_sources: RefCell<Vec<ResourceLoadSource>>,

    class_registry: RefCell<HashMap<JvmString, Class>>,
    array_classes: RefCell<HashMap<ResolvedDescriptor, Class>>,
}

impl fmt::Debug for ClassLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ClassLoader")
            .field("parent", &self.parent())
            .finish()
    }
}

impl ClassLoader {
    pub fn with_parent(
        gc_ctx: GcCtx,
        parent: Option<ClassLoader>,
        backend: Gc<Box<dyn LoaderBackend>>,
    ) -> Self {
        Self(Gc::new(
            gc_ctx,
            ClassLoaderData {
                parent,
                backend,
                load_sources: RefCell::new(Vec::new()),
                class_registry: RefCell::new(HashMap::new()),
                array_classes: RefCell::new(HashMap::new()),
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
        if let Some(element_name) = class_name.strip_prefix('[') {
            // Special handling for array classes
            let element_name = JvmString::new(context.gc_ctx, element_name.to_string());
            let element_descriptor = Descriptor::from_string(context.gc_ctx, element_name)
                .ok_or_else(|| context.no_class_def_found_error(class_name))?;

            let resolved_descriptor =
                ResolvedDescriptor::from_descriptor(context, self, element_descriptor)?;

            let created_class = ClassLoader::array_class_for(context, resolved_descriptor);
            // `array_class_for` will register the class in the correct
            // `ClassLoader`'s registry

            Ok(created_class)
        } else {
            // Not an array class, just recursively lookup on self and ancestors
            let mut current = Some(self);
            while let Some(current_loader) = current {
                let result = current_loader.lookup_own_class(context, class_name)?;
                if let Some(result) = result {
                    return Ok(result);
                }

                current = current_loader.parent();
            }

            Err(context.no_class_def_found_error(class_name))
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
        let data = self.load_resource(None, &full_name);
        if let Some(data) = data {
            let class_file = ClassFile::from_data(context.gc_ctx, data)?;

            let class = Class::from_class_file(&context, self, class_file)?;

            self.register_class(class);

            class.load_methods(context)?;

            Ok(Some(class))
        } else {
            Ok(None)
        }
    }

    pub fn array_class_for(context: &Context, descriptor: ResolvedDescriptor) -> Class {
        let correct_loader = descriptor
            .class()
            .and_then(|c| c.loader())
            .unwrap_or(context.bootstrap_loader());

        correct_loader.get_or_init_array_class(context, descriptor)
    }

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

    pub fn load_resource(
        self,
        class_name: Option<JvmString>,
        resource_name: &String,
    ) -> Option<Vec<u8>> {
        let sources = self.0.load_sources.borrow();
        for source in &*sources {
            let result = self.0.backend.load_resource(
                source,
                class_name.map(|n| n.to_string().clone()),
                resource_name,
            );

            if let Some(result) = result {
                return Some(result);
            }
        }

        None
    }
}

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
    }
}

pub trait LoaderBackend {
    fn load_resource(
        &self,
        load_source: &ResourceLoadSource,
        class_name: Option<String>,
        resource_name: &str,
    ) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub enum ResourceLoadSource {
    // This class was loaded directly from the filesystem. When searching
    // for resources, look at the files in the directory of this class.
    FileSystem,

    // This class was loaded from a JAR file. When searching for resources,
    // look at the files in the directory of this class in the JAR.
    Jar(Jar),
}

impl Trace for ResourceLoadSource {
    fn trace(&self) {
        match self {
            ResourceLoadSource::Jar(jar) => jar.trace(),
            _ => {}
        }
    }
}
