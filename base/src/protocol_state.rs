// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use mina_crypto::hash::Hashable;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::{
    blockchain_state::BlockchainState,
    consensus_state::ConsensusState,
    numbers::{BlockTime, Delta, Length},
};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// Constants that define the consensus parameters
pub struct ProtocolConstants {
    /// Point of finality (number of confirmations)
    pub k: Length,
    /// Number of slots per epoch
    pub slots_per_epoch: Length,
    /// Slots per sub window
    pub slots_per_sub_window: Length,
    /// Maximum permissable delay of packets (in slots after the current)
    pub delta: Delta,
    /// Timestamp of genesis block in unixtime
    pub genesis_state_timestamp: BlockTime,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// This structure can be thought of like the block header. It contains the most essential information of a block.
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: StateHash,
    /// The body of the protocol state
    pub body: ProtocolStateBody,
}

// Protocol state hashes into a StateHash type
impl Hashable<StateHash> for ProtocolState {}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// Body of the protocol state
pub struct ProtocolStateBody {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: StateHash,
    /// Ledger related state
    pub blockchain_state: BlockchainState,
    /// Consensus related state
    pub consensus_state: ConsensusState,
    /// Consensus constants
    pub constants: ProtocolConstants,
}

/// Implementing types have some notion of height and can return it
pub trait Header {
    /// Get the height for the implementing type
    fn get_height(&self) -> Length;
}

impl Header for ProtocolState {
    fn get_height(&self) -> Length {
        self.body.consensus_state.blockchain_length
    }
}
