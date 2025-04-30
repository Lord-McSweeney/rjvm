use rjvm_core::{LoaderBackend, ResourceLoadType};

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
        load_type: &ResourceLoadType,
        class_name: &String,
        resource_name: &String,
    ) -> Option<Vec<u8>> {
        match load_type {
            ResourceLoadType::FileSystem => {
                let mut path_buf = path::PathBuf::from(class_name);
                path_buf.pop();
                path_buf.push(resource_name);

                fs::read(path_buf).ok()
            }
            ResourceLoadType::Jar(jar) => {
                let resolved_name = if let Some(absolute_path) = class_name.strip_prefix('/') {
                    // TODO do absolute paths actually work?
                    absolute_path.to_string()
                } else {
                    // TODO should this handle paths starting with "./", maybe?
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
