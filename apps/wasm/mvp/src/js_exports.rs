// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use js_sys::Uint8Array;
use mina_consensus::common::Chain;
use mina_network::processor::js::graphql_api_v1;
use mina_serialization_types::json::*;
use wasm_bindgen::prelude::*;

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
pub async fn get_best_chain_state_hash() -> Option<String> {
    let frontier = frontier::PROCESSOR.transition_frontier().await;
    let chain = frontier.get_best_chain();
    // TODO: use base58 string
    chain
        .state_hash()
        .map(|state_hash| format!("{:?}", state_hash))
}

#[wasm_bindgen]
pub async fn get_best_chain_state_json() -> Option<String> {
    let frontier = frontier::PROCESSOR.transition_frontier().await;
    let chain = frontier.get_best_chain();
    chain.top().map(|protocol_state| {
        let json: ProtocolStateJson = protocol_state.clone().into();
        serde_json::to_string(&json).unwrap_or_default()
    })
}

#[wasm_bindgen]
pub async fn poll_latest_blocks_once() -> Result<(), JsValue> {
    let backend = frontier::PROCESSOR.nonconsensus_ops().await;
    backend
        .poll_latest_once()
        .await
        .map_err(|err| JsValue::from_str(&format!("{err}")))
}

#[wasm_bindgen]
pub async fn run_processor() {
    frontier::PROCESSOR.run().await;
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
