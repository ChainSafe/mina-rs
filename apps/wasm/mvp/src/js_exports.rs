// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// https://github.com/rustwasm/wasm-bindgen/issues/2774#issuecomment-1030747023
#![allow(clippy::unused_unit)]

use crate::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_event_emitter(e: EventEmitter) {
    e.emit_str("log", "set_event_emitter called in wasm");
    unsafe { EVENT_EMITTER = Some(e) };
}

#[wasm_bindgen]
pub async fn connect(request: Uint8Array) -> Result<Uint8Array, JsValue> {
    let request: pb::requests::ConnectRequest = u8a_to_pb(request).map_err(err_to_js_value)?;
    let response = connect_async(&request).await.map_err(err_to_js_value)?;
    pb_to_u8a(&response).map_err(err_to_js_value)
}
