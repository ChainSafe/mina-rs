// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina consensus state

use crate::{
    common::CompressedPubKeyHashableWrapper,
    epoch_data::EpochData,
    global_slot::GlobalSlot,
    numbers::{Amount, GlobalSlotNumber, Length},
    *,
};
use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};
use mina_serialization_types::{json::*, v1::*, *};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    mina_hasher::{Hashable, ROInput},
    ToChunkedROInput,
};
use proof_systems::{mina_signer::CompressedPubKey, ChunkedROInput};
use smart_default::SmartDefault;
use versioned::*;

/// Wrapper struct for the output for a VRF
#[derive(
    Clone, Default, Eq, PartialEq, Debug, derive_more::From, derive_more::Into, AutoFrom, PartialOrd,
)]
#[auto_from(mina_serialization_types::consensus_state::VrfOutputTruncated)]
#[auto_from(mina_serialization_types::consensus_state::VrfOutputTruncatedBase58Json)]
#[auto_from(mina_serialization_types::consensus_state::VrfOutputTruncatedBase64Json)]
pub struct VrfOutputTruncated(pub Vec<u8>);

impl VrfOutputTruncated {
    /// Calculates blake2b digest
    pub fn digest(&self) -> Vec<u8> {
        let mut hasher = Blake2bVar::new(32).expect("Invalid Blake2bVar output size");
        hasher.update(&self.0);
        hasher.finalize_boxed().to_vec()
    }

    /// From base64 str
    /// TODO: Switch to [From] and [std::fmt::Display] traits
    pub fn from_base64_str(s: &str) -> anyhow::Result<Self> {
        Ok(Self::from_str(s)?)
    }

    /// To base64 string
    /// TODO: Switch to [From] and [std::fmt::Display] traits
    pub fn to_base64_string(&self) -> anyhow::Result<String> {
        let h: VrfOutputTruncatedBase64Json = self.clone().into();
        let json_string = serde_json::to_string(&h)?;
        Ok(serde_json::from_str(&json_string)?)
    }

    /// From base58 str
    /// TODO: Switch to [From] and [std::fmt::Display] traits
    pub fn from_base58_str(s: &str) -> anyhow::Result<Self> {
        let json_string = serde_json::to_string(s)?;
        let json: VrfOutputTruncatedBase58Json = serde_json::from_str(&json_string)?;
        Ok(json.into())
    }

    /// To base58 string
    /// TODO: Switch to [From] and [std::fmt::Display] traits
    pub fn to_base58_string(&self) -> anyhow::Result<String> {
        let h: VrfOutputTruncatedBase58Json = self.clone().into();
        let json_string = serde_json::to_string(&h)?;
        Ok(serde_json::from_str(&json_string)?)
    }
}

impl_strconv_via_json!(VrfOutputTruncated, VrfOutputTruncatedBase64Json);

impl Hashable for VrfOutputTruncated {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        if self.0.len() <= 31 {
            ROInput::new().append_bytes(&self.0)
        } else {
            let roi = ROInput::new().append_bytes(&self.0[..31]);
            if self.0.len() > 31 {
                let last = self.0[31];
                roi.append_bool(last & 0b1 > 0)
                    .append_bool(last & 0b10 > 0)
                    .append_bool(last & 0b100 > 0)
                    .append_bool(last & 0b1000 > 0)
                    .append_bool(last & 0b10000 > 0)
            } else {
                roi
            }
        }
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for VrfOutputTruncated {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        if self.0.len() <= 31 {
            ChunkedROInput::new().append_bytes(&self.0)
        } else {
            let roi = ChunkedROInput::new().append_bytes(&self.0[..31]);
            if self.0.len() > 31 {
                let last = self.0[31];
                roi.append_bool(last & 0b1 > 0)
                    .append_bool(last & 0b10 > 0)
                    .append_bool(last & 0b100 > 0)
                    .append_bool(last & 0b1000 > 0)
                    .append_bool(last & 0b10000 > 0)
            } else {
                roi
            }
        }
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
#[derive(Clone, Debug, Eq, PartialEq, SmartDefault, AutoFrom)]
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

impl ConsensusState {
    /// Returns the sub-window densities as a vec of u32
    pub fn sub_window_densities(&self) -> Vec<u32> {
        self.sub_window_densities.iter().map(|i| i.0).collect()
    }
}

impl FromGraphQLJson for ConsensusState {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self> {
        Ok(Self {
            blockchain_length: json["blockHeight"]
                .as_str()
                .unwrap_or_default()
                .parse::<u32>()?
                .into(),
            epoch_count: json["epochCount"]
                .as_str()
                .unwrap_or_default()
                .parse::<u32>()?
                .into(),
            min_window_density: json["minWindowDensity"]
                .as_str()
                .unwrap_or_default()
                .parse::<u32>()?
                .into(),
            // FIXME: Hard coded?
            sub_window_densities: vec![
                1.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
                7.into(),
            ],
            last_vrf_output: VrfOutputTruncated::from_base58_str(
                json["lastVrfOutput"].as_str().unwrap_or_default(),
            )?,
            total_currency: json["totalCurrency"]
                .as_str()
                .unwrap_or_default()
                .parse::<u64>()?
                .into(),
            curr_global_slot: GlobalSlot {
                slot_number: json["slot"]
                    .as_str()
                    .unwrap_or_default()
                    .parse::<u32>()?
                    .into(),
                // FIXME: Hard coded?
                slots_per_epoch: 7140.into(),
            },
            global_slot_since_genesis: json["slotSinceGenesis"]
                .as_str()
                .unwrap_or_default()
                .parse::<u32>()?
                .into(),
            staking_epoch_data: EpochData::from_graphql_json(&json["stakingEpochData"])?,
            next_epoch_data: EpochData::from_graphql_json(&json["nextEpochData"])?,
            has_ancestor_in_same_checkpoint_window: json["hasAncestorInSameCheckpointWindow"]
                .as_bool()
                .unwrap_or_default(),
            block_stake_winner: CompressedPubKey::from_address(
                json["blockStakeWinner"].as_str().unwrap_or_default(),
            )?,
            block_creator: CompressedPubKey::from_address(
                json["blockCreator"].as_str().unwrap_or_default(),
            )?,
            coinbase_receiver: CompressedPubKey::from_address(
                json["coinbaseReceiever"].as_str().unwrap_or_default(),
            )?,
            supercharge_coinbase: json["superchargedCoinbase"].as_bool().unwrap_or_default(),
        })
    }
}

impl Hashable for ConsensusState {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new()
            .append_hashable(&self.blockchain_length)
            .append_hashable(&self.epoch_count)
            .append_hashable(&self.min_window_density);
        for v in &self.sub_window_densities {
            roi = roi.append_hashable(v);
        }
        roi.append_hashable(&self.last_vrf_output)
            .append_hashable(&self.total_currency)
            .append_hashable(&self.curr_global_slot)
            .append_hashable(&self.global_slot_since_genesis)
            .append_bool(self.has_ancestor_in_same_checkpoint_window)
            .append_bool(self.supercharge_coinbase)
            .append_hashable(&self.staking_epoch_data)
            .append_hashable(&self.next_epoch_data)
            .append_field(self.block_stake_winner.x)
            .append_bool(self.block_stake_winner.is_odd)
            .append_field(self.block_creator.x)
            .append_bool(self.block_creator.is_odd)
            .append_field(self.coinbase_receiver.x)
            .append_bool(self.coinbase_receiver.is_odd)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for ConsensusState {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        let mut roi = ChunkedROInput::new()
            .append_chunked(&self.blockchain_length)
            .append_chunked(&self.epoch_count)
            .append_chunked(&self.min_window_density);
        for l in &self.sub_window_densities {
            roi = roi.append_chunked(l);
        }
        roi.append_chunked(&self.last_vrf_output)
            .append_chunked(&self.total_currency)
            .append_chunked(&self.curr_global_slot)
            .append_chunked(&self.global_slot_since_genesis)
            .append_bool(self.has_ancestor_in_same_checkpoint_window)
            .append_bool(self.supercharge_coinbase)
            .append_chunked(&self.staking_epoch_data)
            .append_chunked(&self.next_epoch_data)
            .append_chunked(&CompressedPubKeyHashableWrapper(&self.block_stake_winner))
            .append_chunked(&CompressedPubKeyHashableWrapper(&self.block_creator))
            .append_chunked(&CompressedPubKeyHashableWrapper(&self.coinbase_receiver))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consensus_state_from_graphql_json() -> anyhow::Result<()> {
        const JSON_STR: &str = r###"
        {
            "blockCreator": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "blockHeight": "1",
            "blockStakeWinner": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "blockchainLength": "1",
            "coinbaseReceiever": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "epoch": "0",
            "epochCount": "0",
            "hasAncestorInSameCheckpointWindow": true,
            "lastVrfOutput": "48FthHHNE1y3YmoS4UvKaM6UyGd9nCJXTPVFQUGak7YtonTDUuEd",
            "minWindowDensity": "77",
            "slot": "0",
            "slotSinceGenesis": "0",
            "superchargedCoinbase": true,
            "totalCurrency": "1013238001000001000",
            "nextEpochData": {
              "epochLength": "2",
              "lockCheckpoint": "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
              "seed": "2vc1zQHJx2xN72vaR4YDH31KwFSr5WHSEH2dzcfcq8jxBPcGiJJA",
              "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
              "ledger": {
                "hash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                "totalCurrency": "1013238001000001000"
              }
            },
            "stakingEpochData": {
              "epochLength": "1",
              "lockCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
              "seed": "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA",
              "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
              "ledger": {
                "hash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                "totalCurrency": "1013238001000001000"
              }
            }
          }
        "###;
        let json = serde_json::from_str(JSON_STR)?;
        _ = ConsensusState::from_graphql_json(&json)?;
        Ok(())
    }

    #[test]
    fn vrf_output_truncated_serde_roundtrip() -> anyhow::Result<()> {
        let b64 = "OruOTtGM3tJL3jM0GHtCzKyugvWT0ZP7VckspHX8_g8=";
        let b58 = "48FthHHNE1y3YmoS4UvKaM6UyGd9nCJXTPVFQUGak7YtonTDUuEd";
        let from_b64 = VrfOutputTruncated::from_base64_str(b64)?;
        let from_b58 = VrfOutputTruncated::from_base58_str(b58)?;
        assert_eq!(from_b64, from_b58);
        assert_eq!(from_b58.to_base64_string()?.as_str(), b64);
        assert_eq!(from_b64.to_base58_string()?.as_str(), b58);
        Ok(())
    }
}
