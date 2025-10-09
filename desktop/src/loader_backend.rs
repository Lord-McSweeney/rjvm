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
        resource_name: &str,
    ) -> Option<Vec<u8>> {
        match load_type {
            ResourceLoadSource::FileSystem => {
                let path_buf = path::PathBuf::from(resource_name);

                fs::read(path_buf).ok()
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
