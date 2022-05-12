// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina consensus state

use crate::{
    common::{U32Json, U64Json},
    epoch_data::EpochDataJson,
    global_slot::GlobalSlotJson,
    signatures::PublicKeyJson,
    v1::*,
};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use versioned::*;

/// Wrapper struct for the output for a VRF
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VrfOutputTruncated(pub Vec<u8>);

/// Wrapper struct for the output for a VRF, with version
pub type VrfOutputTruncatedV1 = Versioned<VrfOutputTruncated, 1>;

/// Wrapper struct for the output for a VRF (json)
#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(VrfOutputTruncated)]
pub struct VrfOutputTruncatedJson(pub Vec<u8>);

impl Serialize for VrfOutputTruncatedJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = base64::encode_config(self.0.as_slice(), base64::URL_SAFE);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for VrfOutputTruncatedJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(
            base64::decode_config(s, base64::URL_SAFE)
                .map_err(<D::Error as serde::de::Error>::custom)?,
        ))
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ConsensusState {
    /// Height of block
    pub blockchain_length: LengthV1,
    /// Epoch number
    pub epoch_count: LengthV1,
    /// Minimum window density oberved on the chain
    pub min_window_density: LengthV1,
    /// Current sliding window of densities
    pub sub_window_densities: Vec<LengthV1>,
    /// Additional VRS output from leader (for seeding Random Oracle)
    pub last_vrf_output: VrfOutputTruncatedV1,
    /// Total supply of currency
    pub total_currency: AmountV1,
    /// Current global slot number relative to the current hard fork
    pub curr_global_slot: GlobalSlotV1,
    /// Absolute global slot number since genesis
    pub global_slot_since_genesis: GlobalSlotNumberV1,
    /// Epoch data for previous epoch
    pub staking_epoch_data: EpochDataV1,
    /// Epoch data for current epoch
    pub next_epoch_data: EpochDataV1,
    /// If the block has an ancestor in the same checkpoint window
    pub has_ancestor_in_same_checkpoint_window: bool,
    /// Compressed public key of winning account
    pub block_stake_winner: PublicKeyV1,
    /// Compressed public key of the block producer
    pub block_creator: PublicKeyV1,
    /// Compresed public key of account receiving the block reward
    pub coinbase_receiver: PublicKeyV1,
    /// true if block_stake_winner has no locked tokens, false otherwise
    pub supercharge_coinbase: bool,
}

/// V1 protocol version of the consensus state
pub type ConsensusStateV1 = Versioned<Versioned<ConsensusState, 1>, 1>;

/// json protocol version of the consensus state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ConsensusState)]
pub struct ConsensusStateJson {
    /// Height of block
    pub blockchain_length: U32Json,
    /// Epoch number
    pub epoch_count: U32Json,
    /// Minimum window density oberved on the chain
    pub min_window_density: U32Json,
    /// Current sliding window of densities
    pub sub_window_densities: Vec<U32Json>,
    /// Additional VRS output from leader (for seeding Random Oracle)
    pub last_vrf_output: VrfOutputTruncatedJson,
    /// Total supply of currency
    pub total_currency: U64Json,
    /// Current global slot number relative to the current hard fork
    pub curr_global_slot: GlobalSlotJson,
    /// Absolute global slot number since genesis
    pub global_slot_since_genesis: U32Json,
    /// Epoch data for previous epoch
    pub staking_epoch_data: EpochDataJson,
    /// Epoch data for current epoch
    pub next_epoch_data: EpochDataJson,
    /// If the block has an ancestor in the same checkpoint window
    pub has_ancestor_in_same_checkpoint_window: bool,
    /// Compressed public key of winning account
    pub block_stake_winner: PublicKeyJson,
    /// Compressed public key of the block producer
    pub block_creator: PublicKeyJson,
    /// Compresed public key of account receiving the block reward
    pub coinbase_receiver: PublicKeyJson,
    /// true if block_stake_winner has no locked tokens, false otherwise
    pub supercharge_coinbase: bool,
}
