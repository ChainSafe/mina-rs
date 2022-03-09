use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace= console, js_name = log)]
    pub fn log_string(s: String);

    #[wasm_bindgen(js_namespace= console, js_name = log)]
    pub fn log_str(s: &str);

    #[wasm_bindgen]
    pub type EventEmitter;

    #[wasm_bindgen(method, js_name = emit)]
    pub fn emit(e: &EventEmitter, event: &str, value: &str);
}
