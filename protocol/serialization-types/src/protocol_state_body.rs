// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use crate::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};
use versioned::*;

/// Body of the protocol state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolStateBody {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: HashV1,
    /// Ledger related state
    pub blockchain_state: BlockchainStateV1,
    /// Consensus related state
    pub consensus_state: ConsensusStateV1,
    /// Consensus constants
    pub constants: ProtocolConstantsV1,
}

/// Body of the protocol state (v1)
pub type ProtocolStateBodyV1 = Versioned<Versioned<ProtocolStateBody, 1>, 1>;

/// Body of the protocol state (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProtocolStateBody)]
pub struct ProtocolStateBodyJson {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: StateHashV1Json,
    /// Ledger related state
    pub blockchain_state: BlockchainStateJson,
    /// Consensus related state
    pub consensus_state: ConsensusStateJson,
    /// Consensus constants
    pub constants: ProtocolConstantsJson,
}
