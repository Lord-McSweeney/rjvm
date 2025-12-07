use rjvm_core::LoaderBackend;

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
    fn load_filesystem_resource(&self, resource_name: &str) -> Option<Vec<u8>> {
        if resource_name == &self.main_class_name {
            Some(self.main_class_data.clone())
        } else {
            None
        }
    }
}
