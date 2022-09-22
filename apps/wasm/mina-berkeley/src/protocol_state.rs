// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::hash::StateHash;
use mina_rs_base::protocol_state::ProtocolState as MinaProtocolState;
use mina_rs_base::types::ProtocolStateBody;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ProtocolState {
    _previous_state_hash: StateHash,
    _body: ProtocolStateBody,
}

impl From<MinaProtocolState> for ProtocolState {
    fn from(v: MinaProtocolState) -> Self {
        ProtocolState {
            _previous_state_hash: v.previous_state_hash,
            _body: v.body,
        }
    }
}
