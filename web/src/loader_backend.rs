use rjvm_core::{LoaderBackend, ResourceLoadSource};

pub struct WebLoaderBackend {
    main_class_name: String,
    main_class_data: Vec<u8>,
}

impl WebLoaderBackend {
    pub fn new(class_name: &str, class_data: &[u8]) -> Self {
        Self {
            main_class_name: class_name.to_string(),
            main_class_data: class_data.to_vec(),
        }
    }
}

impl LoaderBackend for WebLoaderBackend {
    fn load_resource(
        &self,
        load_type: &ResourceLoadSource,
        resource_name: &str,
    ) -> Option<Vec<u8>> {
        match load_type {
            ResourceLoadSource::FileSystem => {
                if resource_name == &self.main_class_name {
                    Some(self.main_class_data.clone())
                } else {
                    None
                }
            }
            ResourceLoadSource::Jar(jar) => {
                if jar.has_file(resource_name) {
                    jar.read_file(resource_name).ok()
                } else {
                    None
                }
            }
        }
    }
}
