// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::{
    epoch_data::EpochData,
    global_slot::GlobalSlot,
    numbers::{self, Amount, Length},
};
use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use serde_versions_derive::version;

#[derive(Clone, Serialize, Deserialize)]
struct VrfOutputTruncated;

/// This structure encapsulates the succinct state of the consensus protocol.
///
/// The stake distribution information is contained by the staking_epoch_data field.
///
/// Due to its succinct nature, Samasika cannot look back into the past to
/// obtain ledger snapshots for the stake distribution. Instead, Samasika implements a novel
/// approach where the future stake distribution snapshot is prepared by the current consensus epoch.
///
/// Samasika prepares the past for the future! This future state is stored in the next_epoch_data field.
#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    /// Height of block
    pub blockchain_length: Length,
    /// Epoch number
    epoch_count: Length,
    /// Minimum odnws density oberved on the chain
    min_window_density: Length,
    /// Current sliding window of densities
    sub_window_densities: Vec<Length>,
    /// Additional VRS output from leader (for seeding Random Oracle)
    last_vrf_output: VrfOutputTruncated,
    /// Total supply of currency
    total_currency: Amount,
    /// Current global slot number relative to the current hard fork
    pub curr_global_slot: GlobalSlot,
    /// Absolute global slot number since genesis
    global_slot_since_genesis: numbers::GlobalSlot,
    /// Epoch data for previous epoch
    staking_epoch_data: EpochData,
    /// Epoch data for current epoch
    next_epoch_data: EpochData,
    has_ancestor_in_same_checkpoint_window: bool,
    /// Compressed public key of winning account
    block_stake_winner: PublicKey,
    /// Compressed public key of the block producer
    block_creator: PublicKey,
    /// Compresed public key of account receiving the block reward
    coinbase_receiver: PublicKey,
    /// true if block_stake_winner has no locked tokens, false otherwise
    supercharge_coinbase: bool,
}
