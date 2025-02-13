use wasm_bindgen::prelude::*;

mod native_impl;
mod runner;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn dbg(s: &str);

    #[wasm_bindgen(js_name = "appendText")]
    pub fn output(s: &str);
}

fn run_file(_name: &str, data: &[u8]) {
    // TODO automatically detect and run JARs
    runner::run_file(data);
}

#[wasm_bindgen(js_name = "fileLoaded")]
pub fn file_loaded(name: &str, data: &[u8]) {
    output(&("rjvm ".to_string() + name + "\n"));
    run_file(name, data);
    output("$ ");
}

#[wasm_bindgen(js_name = "setPanicHook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
