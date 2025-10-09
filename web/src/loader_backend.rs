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
        class_name: Option<String>,
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
                let class_name = if let Some(class_name) = class_name {
                    class_name
                } else {
                    "".to_string()
                };

                let resolved_name = if let Some(absolute_path) = resource_name.strip_prefix('/') {
                    absolute_path.to_string()
                } else {
                    // TODO should this handle paths starting with "./"?
                    let mut path_sections = class_name.split('/').collect::<Vec<_>>();
                    path_sections.pop();
                    path_sections.push(resource_name);

                    path_sections.join("/")
                };

                if jar.has_file(&resolved_name) {
                    jar.read_file(&resolved_name).ok()
                } else {
                    None
                }
            }
        }
    }
}
