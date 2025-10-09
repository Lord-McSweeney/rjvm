use rjvm_core::{LoaderBackend, ResourceLoadSource};

use std::fs;
use std::path;

pub struct DesktopLoaderBackend {}

impl DesktopLoaderBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoaderBackend for DesktopLoaderBackend {
    fn load_resource(
        &self,
        load_type: &ResourceLoadSource,
        class_name: Option<String>,
        resource_name: &str,
    ) -> Option<Vec<u8>> {
        match load_type {
            ResourceLoadSource::FileSystem => {
                let path_buf = if let Some(class_name) = class_name {
                    let mut temp_buf = path::PathBuf::from(class_name);
                    temp_buf.pop();
                    temp_buf.push(resource_name);

                    temp_buf
                } else {
                    path::PathBuf::from(resource_name)
                };

                fs::read(path_buf).ok()
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
