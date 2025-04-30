// Loader trait
use crate::gc::Trace;
use crate::jar::Jar;

pub trait LoaderBackend {
    fn load_resource(
        &self,
        load_type: &ResourceLoadType,
        class_name: &String,
        resource_name: &String,
    ) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub enum ResourceLoadType {
    // This class was loaded directly from the filesystem. When searching
    // for resources, look at the files in the directory of this class.
    FileSystem,

    // This class was loaded from a JAR file. When searching for resources,
    // look at the files in the directory of this class in the JAR.
    Jar(Jar),
}

impl Trace for ResourceLoadType {
    fn trace(&self) {
        match self {
            ResourceLoadType::Jar(jar) => jar.trace(),
            _ => {}
        }
    }
}
