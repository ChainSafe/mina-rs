// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

pub fn err_to_js_error<T: Display>(err: T) -> JsError {
    JsError::new(err.to_string().as_str())
}

pub(crate) fn map_js_err<T: Display>(err: T) -> JsError {
    JsError::new(&format!("{err}"))
}

#[wasm_bindgen(inline_js = r#"
        function js_to_string(v) {
            return `${v}`
        }

        module.exports = {
            js_to_string
        }
    "#)]
extern "C" {
    pub(crate) fn js_to_string(v: &JsValue) -> String;
}
