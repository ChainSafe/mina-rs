// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

pub use mina_network::processor::js::exports::graphql_api_v1;

#[wasm_bindgen]
pub async fn fetch_block_previous_state_hash(
    height: usize,
    state_hash: String,
) -> Result<String, JsValue> {
    let block = graphql_api_v1::fetch_block(height, state_hash.as_str())
        .await
        .map_err(|err| JsValue::from_str(&format!("{}", err)))?;
    block
        .protocol_state
        .previous_state_hash
        .to_base58_string()
        .map_err(|err| JsValue::from_str(&format!("{}", err)))
}

#[wasm_bindgen]
pub fn set_event_emitter(e: EventEmitter) {
    e.emit_str("log", "set_event_emitter called in wasm");
    event_emitter::set_event_emitter(e)
}

#[wasm_bindgen]
pub async fn connect(request: Uint8Array) -> Result<Uint8Array, JsValue> {
    let request: pb::requests::ConnectRequest =
        u8array_to_proto_msg(request).map_err(err_to_js_value)?;
    let response = connect_async(&request).await.map_err(err_to_js_value)?;
    proto_msg_to_u8array(&response).map_err(err_to_js_value)
}
