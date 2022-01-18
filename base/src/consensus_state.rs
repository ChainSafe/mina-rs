// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina consensus state

use crate::{
    epoch_data::EpochData,
    global_slot::GlobalSlot,
    numbers::{Amount, BlockTime, GlobalSlotNumber, Length},
};

use mina_crypto::{base58::Base58Encodable, hash::*, signature::PublicKey};

use derive_more::From;
use mina_crypto::hash::{Hashable, VrfOutputHash};
use mina_crypto::prelude::*;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

/// Wrapper struct for the output for a VRF
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType, From)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct VrfOutputTruncated(pub Vec<u8>);

impl Base64Encodable for VrfOutputTruncated {}

const ERR_FAIL_TO_DECODE_B58: &str = "Failed to decode hash from base58";
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ConsensusState {
    /// Height of block
    pub blockchain_length: Length,
    /// Epoch number
    pub epoch_count: Length,
    /// Minimum windows density oberved on the chain
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
    pub block_stake_winner: PublicKey,
    /// Compressed public key of the block producer
    pub block_creator: PublicKey,
    /// Compresed public key of account receiving the block reward
    pub coinbase_receiver: PublicKey,
    /// true if block_stake_winner has no locked tokens, false otherwise
    pub supercharge_coinbase: bool,
}

pub struct ConsensusConstants {
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

impl Default for ConsensusConstants {
    fn default() -> Self {
        // TODO: read from config
        Self {
            k: Length(290),
            slots_per_epoch: 7140.into(),
            slots_per_sub_window: 7.into(),
            delta: 0.into(),
            genesis_state_timestamp: BlockTime(1615939200000),
            sub_windows_per_window: 11.into(),
            grace_period_end: Length(1440),
        }
    }
}

impl ConsensusState {
    /// Returns the sub-window densities as a vec of u32
    pub fn sub_window_densities(&self) -> Vec<u32> {
        self.sub_window_densities.iter().map(|i| i.0).collect()
    }
}

impl Default for ConsensusState {
    // TODO: read from config
    fn default() -> Self {
        let total_currency = Amount(805385692840039233);
        let staking_epoch_data = {
            let mut data = EpochData::default();

            data.epoch_length.0 = 1;
            data.ledger.hash =
                LedgerHash::from_base58("jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.ledger.total_currency = total_currency;
            data.seed =
                EpochSeed::from_base58("2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.start_checkpoint =
                StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.lock_checkpoint =
                StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data
        };

        let next_epoch_data = {
            let mut data = EpochData::default();
            data.epoch_length.0 = 2;
            data.ledger.hash =
                LedgerHash::from_base58("jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.ledger.total_currency = total_currency;
            data.seed =
                EpochSeed::from_base58("2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.start_checkpoint =
                StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.lock_checkpoint =
                StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data
        };

        Self {
            blockchain_length: Length(1),
            epoch_count: Length(0),
            min_window_density: Length(77),
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
            last_vrf_output: VrfOutputTruncated(vec![13]),
            total_currency,
            curr_global_slot: GlobalSlot {
                slot_number: GlobalSlotNumber(0),
                slots_per_epoch: Length(7140),
            },
            global_slot_since_genesis: GlobalSlotNumber(0),
            staking_epoch_data,
            next_epoch_data,
            has_ancestor_in_same_checkpoint_window: true,
            block_stake_winner: Base58Encodable::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            block_creator: Base58Encodable::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            coinbase_receiver: Base58Encodable::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            supercharge_coinbase: true,
        }
    }
}
