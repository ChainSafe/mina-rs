// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::hash::Hashable;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::{
    blockchain_state::BlockchainState,
    consensus_state::ConsensusState,
    numbers::{BlockTime, Delta, Length},
};
use mina_crypto::hash::StateHash;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ProtocolConstants {
    k: Length,
    pub slots_per_epoch: Length,
    slots_per_sub_window: Length,
    delta: Delta,
    genesis_state_timestamp: BlockTime,
}

/// This structure can be thought of like the block header. It contains the most essential information of a block.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ProtocolState {
    pub previous_state_hash: StateHash,
    pub body: ProtocolStateBody,
}

// Protocol state hashes into a StateHash type
impl Hashable<StateHash> for ProtocolState {}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ProtocolStateBody {
    pub genesis_state_hash: StateHash,
    pub blockchain_state: BlockchainState,
    pub consensus_state: ConsensusState,
    pub constants: ProtocolConstants,
}

pub trait Header {
    fn get_height(&self) -> Length;
}

impl Header for ProtocolState {
    fn get_height(&self) -> Length {
        self.body.consensus_state.blockchain_length
    }
}
