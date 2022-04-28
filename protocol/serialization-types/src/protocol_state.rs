// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::v1::{HashV1, ProtocolStateBodyV1};

/// This structure can be thought of like the block header. It contains the most essential information of a block.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: HashV1,
    /// The body of the protocol state
    pub body: ProtocolStateBodyV1,
}

/// This structure can be thought of like the block header. It contains the most essential information of a block (v1)
pub type ProtocolStateV1 = Versioned<Versioned<ProtocolState, 1>, 1>;

/// This structure can be thought of like the block header. It contains the most essential information of a block.
/// that is convertible from / to the mina specific json representation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolStateJson {}

impl From<ProtocolStateJson> for ProtocolStateV1 {
    fn from(t: ProtocolStateJson) -> Self {
        let t: ProtocolState = t.into();
        t.into()
    }
}

impl From<ProtocolStateV1> for ProtocolStateJson {
    fn from(t: ProtocolStateV1) -> Self {
        t.t.t.into()
    }
}

impl From<ProtocolStateJson> for ProtocolState {
    fn from(_t: ProtocolStateJson) -> Self {
        unimplemented!()
    }
}

impl From<ProtocolState> for ProtocolStateJson {
    fn from(_t: ProtocolState) -> Self {
        unimplemented!()
    }
}
