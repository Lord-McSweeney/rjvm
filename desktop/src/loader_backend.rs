use rjvm_core::LoaderBackend;

use std::fs;
use std::path;

pub struct DesktopLoaderBackend {}

impl DesktopLoaderBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoaderBackend for DesktopLoaderBackend {
    fn load_filesystem_resource(&self, resource_name: &str) -> Option<Vec<u8>> {
        let path_buf = path::PathBuf::from(resource_name);

        fs::read(path_buf).ok()
    }
}
