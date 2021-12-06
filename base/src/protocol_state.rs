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
    /// Point of finality (number of confirmations)
    pub k: Length,
    /// Number of slots per epoch
    pub slots_per_epoch: Length,
    /// No of slots in a sub-window = 7
    pub slots_per_sub_window: Length,
    /// Maximum permissable delay of packets (in slots after the current)
    pub delta: Length,
    /// Timestamp of genesis block in unixtime
    pub genesis_state_timestamp: BlockTime,
    /// Sub windows within a window
    pub sub_windows_per_window: Length,
    /// Number of slots before minimum density is used in chain selection
    pub grace_period_end: Length,
}

impl ProtocolConstants {
    pub fn new() -> Self {
        // if cfg!(feature = "mainnet") {
        let slots_per_sub_window = 7;
        let sub_windows_per_window = 11;
        // let slot_duration_ms = 180000;
        // let one_year_ms = 31556952000;
        // let checkpoint_window_slots_per_year = one_year_ms / slot_duration_ms;
        Self {
            k: Length(290),
            slots_per_epoch: Length(7140),
            slots_per_sub_window: Length(slots_per_sub_window),
            delta: Length(0),
            genesis_state_timestamp: BlockTime(1615939200000),
            sub_windows_per_window: Length(sub_windows_per_window),
            grace_period_end: Length(1440),
            // slot_duration_ms: BlockTimeSpan(slot_duration_ms),
            // epoch_duration: BlockTimeSpan(1285200000),
            // acceptable_network_delay: Length(180000),
            // slots_per_window: Length(slots_per_sub_window * sub_windows_per_window),
            // checkpoint_window_slots_per_year: Length(checkpoint_window_slots_per_year),
            // checkpoint_window_size_in_slots: Length(checkpoint_window_slots_per_year / 12),
            // epoch_size: todo!(),
            // block_window_duration_ms: todo!(),
            // delta_duration: todo!()
        }
        // } else { // devnet
        // let slots_per_sub_window = 7;
        // let sub_windows_per_window = 11;
        // Self {
        //     k: Length(290),
        //     slots_per_epoch: Length(7140),
        //     slots_per_sub_window: Length(slots_per_sub_window),
        //     delta: Length(0),
        //     genesis_state_timestamp: BlockTime(1615939200000),
        //     sub_windows_per_window: Length(sub_windows_per_window),
        //     grace_period_end: Length(1440),
        //     // slot_duration_ms: BlockTimeSpan(180000),
        //     // epoch_duration: BlockTimeSpan(1285200000),
        //     // acceptable_network_delay: Length(180000),
        //     // sub_windows_per_window: Length(sub_windows_per_window),
        //     // slots_per_window: Length(slots_per_sub_window * sub_windows_per_window),
        //     // epoch_size: todo!(),
        //     // checkpoint_window_slots_per_year: todo!(),
        //     // checkpoint_window_size_in_slots: todo!(),
        //     // block_window_duration_ms: todo!(),
        //     // delta_duration: todo!()
        // }
        // }
    }
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
