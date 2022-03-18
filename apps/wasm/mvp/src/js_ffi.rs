// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// https://github.com/rustwasm/wasm-bindgen/issues/2774#issuecomment-1030747023
#![allow(clippy::unused_unit)]

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
