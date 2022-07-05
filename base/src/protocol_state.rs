// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use crate::{
    blockchain_state::BlockchainState,
    consensus_state::ConsensusState,
    global_slot::GlobalSlot,
    numbers::{BlockTime, Length},
};
use lockfree_object_pool::SpinLockObjectPool;
use mina_crypto::hash::StateHash;
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use once_cell::sync::OnceCell;
use proof_systems::mina_hasher::{
    create_legacy, Fp, Hashable, Hasher, PoseidonHasherLegacy, ROInput,
};
use versioned::*;

/// Constants that define the consensus parameters
#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
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
        let mut roi = ROInput::new();
        roi.append_hashable(&self.k);
        roi.append_hashable(&self.delta);
        roi.append_hashable(&self.slots_per_epoch);
        roi.append_hashable(&self.slots_per_sub_window);
        roi.append_hashable(&self.genesis_state_timestamp);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

#[derive(Clone, Default, Debug, PartialEq, AutoFrom)]
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
        static HASHER_POOL: OnceCell<SpinLockObjectPool<PoseidonHasherLegacy<ProtocolStateBody>>> =
            OnceCell::new();
        let pool =
            HASHER_POOL.get_or_init(|| SpinLockObjectPool::new(|| create_legacy(()), |_| ()));
        let mut hasher = pool.pull();
        let body_hash = hasher.hash(&self.body);

        let mut roi = ROInput::new();
        roi.append_hashable(&self.previous_state_hash);
        roi.append_field(body_hash);
        roi
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
        static HASHER_POOL: OnceCell<SpinLockObjectPool<PoseidonHasherLegacy<ProtocolState>>> =
            OnceCell::new();
        let pool =
            HASHER_POOL.get_or_init(|| SpinLockObjectPool::new(|| create_legacy(()), |_| ()));
        let mut hasher = pool.pull();
        hasher.hash(self)
    }

    /// Calculates the state hash of current protocol state
    pub fn state_hash(&self) -> StateHash {
        let f = self.state_hash_fp();
        (&f).into()
    }
}

#[derive(Clone, Default, Debug, PartialEq, AutoFrom)]
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
        let mut roi = ROInput::new();
        roi.append_hashable(&self.blockchain_state);
        roi.append_hashable(&self.consensus_state);
        roi.append_hashable(&self.genesis_state_hash);
        roi.append_hashable(&self.constants);
        roi
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
