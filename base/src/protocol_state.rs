// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use crate::{
    blockchain_state::BlockchainState,
    consensus_state::ConsensusState,
    global_slot::GlobalSlot,
    numbers::{BlockTime, Length},
};
use mina_crypto::hash::StateHash;
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_hasher::{create_legacy, Fp, Hashable, Hasher, ROInput};
use versioned::*;

/// Constants that define the consensus parameters
#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_constants::ProtocolConstants)]
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

impl_from_with_proxy!(
    ProtocolConstants,
    ProtocolConstantsV1,
    ProtocolConstantsJson
);

impl Hashable for ProtocolConstants {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.k)
            .append_hashable(&self.delta)
            .append_hashable(&self.slots_per_epoch)
            .append_hashable(&self.slots_per_sub_window)
            .append_hashable(&self.genesis_state_timestamp)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state::ProtocolState)]
/// This structure can be thought of like the block header. It contains the most essential information of a block.
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: StateHash,
    /// The body of the protocol state
    pub body: ProtocolStateBody,
}

impl_from_with_proxy!(ProtocolState, ProtocolStateV1, ProtocolStateJson);

impl Hashable for ProtocolState {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut hasher = create_legacy(());
        let body_hash = hasher.hash(&self.body);
        ROInput::new()
            .append_hashable(&self.previous_state_hash)
            .append_field(body_hash)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaProtoState".into())
    }
}

impl ProtocolState {
    /// Gets the current global slot the current epoch
    pub fn curr_global_slot(&self) -> &GlobalSlot {
        &self.body.consensus_state.curr_global_slot
    }

    /// Calculates the state hash field of current protocol state
    pub fn state_hash_fp(&self) -> Fp {
        let mut hasher = create_legacy(());
        hasher.hash(self)
    }

    /// Calculates the state hash of current protocol state
    pub fn state_hash(&self) -> StateHash {
        let f = self.state_hash_fp();
        (&f).into()
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_body::ProtocolStateBody)]
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

impl_from_with_proxy!(
    ProtocolStateBody,
    ProtocolStateBodyV1,
    ProtocolStateBodyJson
);

impl Hashable for ProtocolStateBody {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.constants)
            .append_hashable(&self.genesis_state_hash)
            .append_hashable(&self.blockchain_state)
            .append_hashable(&self.consensus_state)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaProtoStateBody".into())
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
