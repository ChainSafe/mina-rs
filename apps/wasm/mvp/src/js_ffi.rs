// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use js_sys::Uint8Array;
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
    pub fn emit_str(e: &EventEmitter, event: &str, value: &str);

    #[wasm_bindgen(method, js_name = emit)]
    pub fn emit_u8a(e: &EventEmitter, event: &str, value: &Uint8Array);
}
