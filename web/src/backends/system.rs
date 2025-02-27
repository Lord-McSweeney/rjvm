use rjvm_core::SystemBackend;

pub struct WebSystemBackend {}

impl WebSystemBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl SystemBackend for WebSystemBackend {
    fn exit(&self, exit_code: i32) -> ! {
        // No exit function on web
        panic!("System.exit called (code {})", exit_code)
    }
}
