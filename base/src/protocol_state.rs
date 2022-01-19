// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use mina_crypto::hash::{Hashable, StateHash};
use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::{
    blockchain_state::BlockchainState,
    consensus_state::ConsensusState,
    global_slot::GlobalSlot,
    numbers::{BlockTime, Length},
    network_types::{ProtocolStateV1, ProtocolStateBodyV1, ProtocolConstantsV1},
    network_types,
};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
/// Constants that define the consensus parameters
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
}

impl From<ProtocolConstantsV1> for ProtocolConstants {
    fn from(t: ProtocolConstantsV1) -> Self {
        let t = t.0.inner().inner();
        Self {
            k: t.k,
            slots_per_epoch: t.slots_per_epoch,
            slots_per_sub_window: t.slots_per_sub_window,
            delta: t.delta,
            genesis_state_timestamp: t.genesis_state_timestamp,
        }
    }
}

impl Into<ProtocolConstantsV1> for ProtocolConstants {
    fn into(self) -> ProtocolConstantsV1 {
        ProtocolConstantsV1(
            Versioned::new(
                Versioned::new(
                    network_types::protocol_constants::ProtocolConstants {
                        k: self.k,
                        slots_per_epoch: self.slots_per_epoch,
                        slots_per_sub_window: self.slots_per_sub_window,
                        delta: self.delta,
                        genesis_state_timestamp: self.genesis_state_timestamp,
                    }
                )
            )
        )
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
/// This structure can be thought of like the block header. It contains the most essential information of a block.
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: StateHash,
    /// The body of the protocol state
    pub body: ProtocolStateBody,
}

impl ProtocolState {
    /// Gets the current global slot the current epoch
    pub fn curr_global_slot(&self) -> &GlobalSlot {
        &self.body.consensus_state.curr_global_slot
    }
}

impl From<ProtocolStateV1> for ProtocolState {
    fn from(t: ProtocolStateV1) -> Self {
        let t = t.0.inner().inner();
        Self {
            previous_state_hash: t.previous_state_hash,
            body: t.body.into(),
        }
    }
}

impl Into<ProtocolStateV1> for ProtocolState {
    fn into(self) -> ProtocolStateV1 {
        ProtocolStateV1(
            Versioned::new(
                Versioned::new(
                    network_types::protocol_state::ProtocolState {
                      previous_state_hash: self.previous_state_hash,
                      body: self.body.into(),
                    }
                )
            )
        )
    }
}

// Protocol state hashes into a StateHash type
impl Hashable<StateHash> for ProtocolState {}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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

impl From<ProtocolStateBodyV1> for ProtocolStateBody {
    fn from(t: ProtocolStateBodyV1) -> Self {
        let t = t.0.inner().inner();
        Self {
            genesis_state_hash: t.genesis_state_hash,
            blockchain_state: t.blockchain_state,
            consensus_state: t.consensus_state,
            constants: t.constants.into(),
        }
    }
}

impl Into<ProtocolStateBodyV1> for ProtocolStateBody {
    fn into(self) -> ProtocolStateBodyV1 {
        ProtocolStateBodyV1(
            Versioned::new(
                Versioned::new(
                    network_types::protocol_state_body::ProtocolStateBody {
                        genesis_state_hash: self.genesis_state_hash,
                        blockchain_state: self.blockchain_state,
                        consensus_state: self.consensus_state,
                        constants: self.constants.into(),
                    }
                )
            )
        )
    }
}

/// Implementing types have some notion of height and can return it
pub trait Header {
    /// Get the height for the implementing type
    fn get_height(&self) -> Length;
    /// The minimum window density at the current epoch.
    fn min_window_density(&self) -> Length;
    /// A list of density values of the sub windows.
    fn sub_window_densities(&self) -> &Vec<Length>;
}

impl Header for ProtocolState {
    fn get_height(&self) -> Length {
        self.body.consensus_state.blockchain_length
    }

    fn sub_window_densities(&self) -> &Vec<Length> {
        &self.body.consensus_state.sub_window_densities
    }

    fn min_window_density(&self) -> Length {
        self.body.consensus_state.min_window_density
    }
}
