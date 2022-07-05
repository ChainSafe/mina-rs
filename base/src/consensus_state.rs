// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina consensus state

use crate::{
    epoch_data::EpochData,
    global_slot::GlobalSlot,
    numbers::{Amount, GlobalSlotNumber, Length},
};
use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};
use mina_serialization_types::{json::*, v1::*, *};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_hasher::{Hashable, ROInput};
use proof_systems::mina_signer::CompressedPubKey;
use smart_default::SmartDefault;
use versioned::*;

/// Wrapper struct for the output for a VRF
#[derive(
    Clone, Default, PartialEq, Debug, derive_more::From, derive_more::Into, AutoFrom, PartialOrd,
)]
#[auto_from(mina_serialization_types::consensus_state::VrfOutputTruncated)]
#[auto_from(mina_serialization_types::consensus_state::VrfOutputTruncatedJson)]
pub struct VrfOutputTruncated(pub Vec<u8>);

impl VrfOutputTruncated {
    /// Calculates blake2b digest
    pub fn digest(&self) -> Vec<u8> {
        let mut hasher = Blake2bVar::new(32).expect("Invalid Blake2bVar output size");
        hasher.update(&self.0);
        hasher.finalize_boxed().to_vec()
    }
}

impl_strconv_via_json!(VrfOutputTruncated, VrfOutputTruncatedJson);

impl Hashable for VrfOutputTruncated {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_bytes(&self.digest());
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

/// This structure encapsulates the succinct state of the consensus protocol.
///
/// The stake distribution information is contained by the staking_epoch_data field.
///
/// Due to its succinct nature, Samasika cannot look back into the past to
/// obtain ledger snapshots for the stake distribution. Instead, Samasika implements a novel
/// approach where the future stake distribution snapshot is prepared by the current consensus epoch.
///
/// Samasika prepares the past for the future! This future state is stored in the next_epoch_data field.
#[derive(Clone, Debug, PartialEq, SmartDefault, AutoFrom)]
#[auto_from(mina_serialization_types::consensus_state::ConsensusState)]
pub struct ConsensusState {
    /// Height of block
    pub blockchain_length: Length,
    /// Epoch number
    pub epoch_count: Length,
    /// Minimum window density oberved on the chain
    pub min_window_density: Length,
    /// Current sliding window of densities
    pub sub_window_densities: Vec<Length>,
    /// Additional VRS output from leader (for seeding Random Oracle)
    pub last_vrf_output: VrfOutputTruncated,
    /// Total supply of currency
    pub total_currency: Amount,
    /// Current global slot number relative to the current hard fork
    pub curr_global_slot: GlobalSlot,
    /// Absolute global slot number since genesis
    pub global_slot_since_genesis: GlobalSlotNumber,
    /// Epoch data for previous epoch
    pub staking_epoch_data: EpochData,
    /// Epoch data for current epoch
    pub next_epoch_data: EpochData,
    /// If the block has an ancestor in the same checkpoint window
    pub has_ancestor_in_same_checkpoint_window: bool,
    /// Compressed public key of winning account
    #[default(CompressedPubKey::from_address("B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg").unwrap())]
    pub block_stake_winner: CompressedPubKey,
    /// Compressed public key of the block producer
    #[default(CompressedPubKey::from_address("B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg").unwrap())]
    pub block_creator: CompressedPubKey,
    /// Compresed public key of account receiving the block reward
    #[default(CompressedPubKey::from_address("B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg").unwrap())]
    pub coinbase_receiver: CompressedPubKey,
    /// true if block_stake_winner has no locked tokens, false otherwise
    pub supercharge_coinbase: bool,
}

impl_from_with_proxy!(ConsensusState, ConsensusStateV1, ConsensusStateJson);

impl Hashable for ConsensusState {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.blockchain_length);
        roi.append_hashable(&self.epoch_count);
        roi.append_hashable(&self.min_window_density);
        for v in &self.sub_window_densities {
            roi.append_hashable(v);
        }
        roi.append_hashable(&self.last_vrf_output);
        roi.append_hashable(&self.total_currency);
        roi.append_hashable(&self.curr_global_slot);
        roi.append_hashable(&self.global_slot_since_genesis);
        roi.append_bool(self.has_ancestor_in_same_checkpoint_window);
        roi.append_bool(self.supercharge_coinbase);
        roi.append_hashable(&self.staking_epoch_data);
        roi.append_hashable(&self.next_epoch_data);
        roi.append_field(self.block_stake_winner.x);
        roi.append_bool(self.block_stake_winner.is_odd);
        roi.append_field(self.block_creator.x);
        roi.append_bool(self.block_creator.is_odd);
        roi.append_field(self.coinbase_receiver.x);
        roi.append_bool(self.coinbase_receiver.is_odd);

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ConsensusState {
    /// Returns the sub-window densities as a vec of u32
    pub fn sub_window_densities(&self) -> Vec<u32> {
        self.sub_window_densities.iter().map(|i| i.0).collect()
    }
}
