// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::v1::{HashV1, ProtocolStateBodyV1};

/// This structure can be thought of like the block header. It contains the most essential information of a block.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: HashV1,
    /// The body of the protocol state
    pub body: ProtocolStateBodyV1,
}

/// This structure can be thought of like the block header. It contains the most essential information of a block (v1)
pub type ProtocolStateV1 = Versioned<Versioned<ProtocolState, 1>, 1>;
