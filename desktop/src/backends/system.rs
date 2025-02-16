use rjvm_core::SystemBackend;

use std::process;

pub struct DesktopSystemBackend {}

impl DesktopSystemBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl SystemBackend for DesktopSystemBackend {
    fn exit(&self, exit_code: i32) -> ! {
        process::exit(exit_code)
    }
}
