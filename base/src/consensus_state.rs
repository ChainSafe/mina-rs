// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina consensus state

use crate::{
    epoch_data::EpochData,
    global_slot::GlobalSlot,
    numbers::{Amount, GlobalSlotNumber, Length},
};
use derive_more::From;
use mina_crypto::{hash::*, prelude::*};
use mina_signer::CompressedPubKey;
use serde::Serialize;


/// Wrapper struct for the output for a VRF
#[derive(Clone, Default, PartialEq, Debug, From, Serialize)]
pub struct VrfOutputTruncated(pub Vec<u8>);

impl Base64Encodable for VrfOutputTruncated {}

impl From<&str> for VrfOutputTruncated {
    fn from(s: &str) -> Self {
        VrfOutputTruncated(s.as_bytes().to_vec())
    }
}

impl Hashable<VrfOutputHash> for VrfOutputTruncated {}

impl AsRef<[u8]> for VrfOutputTruncated {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
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
#[derive(Clone, Debug, PartialEq)]
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
    pub block_stake_winner: CompressedPubKey,
    /// Compressed public key of the block producer
    pub block_creator: CompressedPubKey,
    /// Compresed public key of account receiving the block reward
    pub coinbase_receiver: CompressedPubKey,
    /// true if block_stake_winner has no locked tokens, false otherwise
    pub supercharge_coinbase: bool,
}

impl ConsensusState {
    /// Returns the sub-window densities as a vec of u32
    pub fn sub_window_densities(&self) -> Vec<u32> {
        self.sub_window_densities.iter().map(|i| i.0).collect()
    }
}
