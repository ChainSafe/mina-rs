// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::network_types::v1::ProtocolStateBodyV1;
use mina_crypto::hash::StateHash;

/// This structure can be thought of like the block header. It contains the most essential information of a block.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: StateHash,
    /// The body of the protocol state
    pub body: ProtocolStateBodyV1,
}

/// versioned wrapper
pub type ProtocolStateV1 = Versioned<Versioned<ProtocolState, 1>, 1>;
