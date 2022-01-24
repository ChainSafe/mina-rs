// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use mina_crypto::hash::StateHash;
use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::network_types::v1::ProtocolConstantsV1;
use crate::{blockchain_state::BlockchainState, consensus_state::ConsensusState};

/// Body of the protocol state
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolStateBody {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: StateHash,
    /// Ledger related state
    pub blockchain_state: BlockchainState,
    /// Consensus related state
    pub consensus_state: ConsensusState,
    /// Consensus constants
    pub constants: ProtocolConstantsV1,
}

/// versioned wrapper
pub type ProtocolStateBodyV1 = Versioned<Versioned<ProtocolStateBody, 1>, 1>;
