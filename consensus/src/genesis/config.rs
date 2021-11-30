// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::{
    base58::Base58Encodable,
    hash::{EpochSeed, LedgerHash, StateHash},
    signature::PublicKey,
};
use mina_rs_base::types::*;

const ERR_FAIL_TO_DECODE_B58: &str = "Failed to decode ledger hash from base58";

lazy_static::lazy_static! {
    pub static ref MAINNET_CONFIG: GenesisInitConfig = GenesisInitConfig::mainnet();
    pub static ref DEVNET_CONFIG: GenesisInitConfig = GenesisInitConfig::devnet();
}

pub struct GenesisInitConfig {
    pub(crate) constants: ProtocolConstants,

    pub(crate) sub_windows_per_window: u32,
    pub(crate) staking_epoch_data: EpochData,
    pub(crate) next_epoch_data: EpochData,
    pub(crate) block_stake_winner: PublicKey,
    pub(crate) block_creator: PublicKey,
    pub(crate) coinbase_receiver: PublicKey,
    pub(crate) genesis_state_hash: StateHash,
    pub(crate) previous_state_hash: StateHash,
    pub(crate) blockchain_state: BlockchainState,
}

impl GenesisInitConfig {
    pub(crate) fn mainnet() -> Self {
        // https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#3-constants
        let constants = ProtocolConstants {
            k: 290.into(),
            slots_per_epoch: 7140.into(),
            slots_per_sub_window: 7.into(),
            delta: 0.into(),
            genesis_state_timestamp: BlockTime::from_unix_timestamp(1615939200000),
        };

        let staking_epoch_data = {
            let mut data = EpochData::default();
            data.epoch_length.0 = 1;
            data.ledger.hash =
                LedgerHash::from_base58("jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee")
                    .expect(ERR_FAIL_TO_DECODE_B58);
            data.ledger.total_currency = Amount(805385692840039300);
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
            data.ledger.total_currency = Amount(805385692840039300);
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

        let blockchain_state = BlockchainState {
            timestamp: BlockTime::from_unix_timestamp(1615939200000),
            // staged_ledger_hash:
            ..Default::default()
        };

        Self {
            sub_windows_per_window: 11,
            constants,
            staking_epoch_data,
            next_epoch_data,
            block_stake_winner: PublicKey::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            block_creator: PublicKey::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            coinbase_receiver: PublicKey::from_base58(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            genesis_state_hash: StateHash::from_base58(
                "3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            previous_state_hash: StateHash::from_base58(
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            blockchain_state,
        }
    }

    pub(crate) fn devnet() -> Self {
        // FIXME: Figure out devnet config
        Self::mainnet()
    }
}
