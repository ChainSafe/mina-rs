// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::{
    base58::{version_bytes, Base58Encodable, Base58EncodableHash},
    hash::*,
    signature::PublicKey,
};
use mina_rs_base::types::*;

const ERR_FAIL_TO_DECODE_B58: &str = "Failed to decode hash from base58";
const ERR_FAIL_TO_DECODE_B64: &str = "Failed to decode hash from base64";
const ERR_FAIL_TO_DECODE_HEX: &str = "Failed to decode hash from hex";

lazy_static::lazy_static! {
    pub static ref MAINNET_CONFIG: GenesisInitConfig = GenesisInitConfig::mainnet();
    pub static ref DEVNET_CONFIG: GenesisInitConfig = GenesisInitConfig::devnet();
}

pub struct GenesisInitConfig {
    pub(crate) constants: ProtocolConstants,

    pub(crate) sub_windows_per_window: u32,
    pub(crate) last_vrf_output: VrfOutputTruncated,
    pub(crate) total_currency: Amount,
    pub(crate) sub_window_densities: Vec<Length>,
    pub(crate) staking_epoch_data: EpochData,
    pub(crate) next_epoch_data: EpochData,
    pub(crate) block_stake_winner: PublicKey,
    pub(crate) block_creator: PublicKey,
    pub(crate) coinbase_receiver: PublicKey,
    pub(crate) genesis_state_hash: StateHash,
    pub(crate) previous_state_hash: StateHash,
    pub(crate) blockchain_state: BlockchainState,
    pub(crate) protocol_state_proof: ProtocolStateProof,
    pub(crate) delta_transition_chain_proof: DeltaTransitionChainProof,
}

impl GenesisInitConfig {
    pub(crate) fn mainnet() -> Self {
        // https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#3-constants
        let constants = ProtocolConstants {
            k: 290.into(),
            slots_per_epoch: 7140.into(),
            slots_per_sub_window: 7.into(),
            delta: 0.into(),
            genesis_state_timestamp: BlockTime::from_unix_epoch(1615939200),
        };
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

        let blockchain_state = BlockchainState {
            timestamp: BlockTime::from_unix_epoch(1615939200),
            snarked_next_available_token: TokenId(2),
            snarked_ledger_hash: SnarkedLedgerHash::from_base58(
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            genesis_ledger_hash: SnarkedLedgerHash::from_base58(
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            staged_ledger_hash: StagedLedgerHash {
                non_snark: NonSnarkStagedLedgerHash {
                    ledger_hash: LedgerHash::from_base58(
                        "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee",
                    )
                    .expect(ERR_FAIL_TO_DECODE_B58),
                    aux_hash: AuxHash(decode_aux_hash_from_base58(
                        "UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom",
                        version_bytes::STAGED_LEDGER_HASH_AUX_HASH,
                    )),
                    pending_coinbase_aux: AuxHash(decode_aux_hash_from_base58(
                        "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
                        version_bytes::STAGED_LEDGER_HASH_PENDING_COINBASE_AUX,
                    )),
                },
                pending_coinbase_hash: CoinBaseHash::from_base58(
                    "2n1tLdP2gkifmyVmrmzYXTS4ohPbZPJn6Qq4x55ywrbRWB4543cC",
                )
                .expect(ERR_FAIL_TO_DECODE_B58),
            },
        };

        let protocol_state_proof = {
            let mut p = ProtocolStateProof::default();

            let ev0 = &mut p.proof.openings.evals.0;
            ev0.l = FieldElementVec::try_from_hex_string(
                "0x2e53605b801ad7fea745e9766add8da9ed33589d758fb339fed40c329c59aa27",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.r = FieldElementVec::try_from_hex_string(
                "0xb77a8788b07f7cd1c9c61618755cca3d0d303a7b096124ce0c02dc5f451a0f03",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.o = FieldElementVec::try_from_hex_string(
                "0x2e1e68731d00b84720038823777ec6522d9a1e9e365920c3e7ce064ade0c2e1e",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.z = FieldElementVec::try_from_hex_string(
                "0xd96d62e54a0a49d3a44c919eb4b089333d64a236edcda1921274ac6903bad937",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.t = FieldElementVec::try_from_hex_string(
                [
                    "717115e59713c84f88babe2ec0292518060d2cc82b54e9a9c9a2d2a87ce91e15",
                    "6994e270f284a557c418afebfaaca2794c8af6a476cb1b9478c205e8a901170f",
                    "d82d38717842bde317157edf186a5b2a5ac2a035a069b18a1bb790d8a1b60e26",
                    "c37d692c8473aa9a246bb85e5c4323cd0c5a69e4b9ce1ae160f961447c31ae2e",
                    "cce3a78dfa242d8c53e89467cc986dfd332db987d76c66e7735a47ef34e90f28",
                ]
                .join(""),
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.f = FieldElementVec::try_from_hex_string(
                "0x5dd93c9b2c3fcee30fa34960f2472fcd04d9de8486f635c9b96d776fae31221f",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.sigma1 = FieldElementVec::try_from_hex_string(
                "0xa84f94a0d6d64be0b97049b92ae2c58a8cb93e792179fab57fa32c4695abe724",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev0.sigma2 = FieldElementVec::try_from_hex_string(
                "0x2c7c6aa5123b41aa8eace85a7eeeb8ebb22219c9353b9276711199aaa8018217",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);

            let ev1 = &mut p.proof.openings.evals.1;
            ev1.l = FieldElementVec::try_from_hex_string(
                "0x16eba2ebda9feac442e29ef9293f5c4576933d531a6e3c07518e352241055f3d",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.r = FieldElementVec::try_from_hex_string(
                "0xdcf5b2e12453b8369c420e76ada0fb6c6e173f2271aa19ec6db8010112611605",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.o = FieldElementVec::try_from_hex_string(
                "0x35362d986f20c598e53c3de0b8fc41300484243172af893cc99ca199aa16163c",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.z = FieldElementVec::try_from_hex_string(
                "0xf0951e6a385fb4ea8b5e2cf0e89e54807a99938b0ab69c77f1b9b210a05d152e",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.t = FieldElementVec::try_from_hex_string(
                [
                    "b5c98d3a881eaad5600d89920dff83025079d27bde3ceadd14425bfc8a40d310",
                    "ee802beaf4ddbaf3b69698689d7e76b670caa65ddbd92197227ab0c8dfba3624",
                    "6bdf230ec07a915319c606ad930c41dd7f097222ada2776a484e755feb2d491c",
                    "e00d36cc2b6076c23184046c0a2a062085215644fe29549a6252025055bdfb1c",
                    "37befa9d80c628fb8b3f7f5316912c175426a0ad9a83db780847d636f1ccab09",
                ]
                .join(""),
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.f = FieldElementVec::try_from_hex_string(
                "0xc0441628012519d76fef0107434dc56bb174e7d1610cde2fc86d6aa72b75ad1a",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.sigma1 = FieldElementVec::try_from_hex_string(
                "0xcd71c8afe1a719f2e5e83fce7941fb9a313e2b9262480afa68675dcfab64b20a",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);
            ev1.sigma2 = FieldElementVec::try_from_hex_string(
                "0xe580093d240406f6684b313ce40669bd5ba1c8df3ed53ced2f473c037af19a08",
            )
            .expect(ERR_FAIL_TO_DECODE_HEX);

            p
        };

        Self {
            sub_windows_per_window: 11,
            last_vrf_output: VrfOutputTruncated::try_from_base64(
                "NfThG1r1GxQuhaGLSJWGxcpv24SudtXG4etB0TnGqwg=",
            )
            .expect(ERR_FAIL_TO_DECODE_B64),
            total_currency,
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
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            previous_state_hash: StateHash::from_base58(
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d",
            )
            .expect(ERR_FAIL_TO_DECODE_B58),
            blockchain_state,
            protocol_state_proof,
            delta_transition_chain_proof: (
                StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
                    .expect(ERR_FAIL_TO_DECODE_B58),
                Default::default(),
            ),
        }
    }

    pub(crate) fn devnet() -> Self {
        // FIXME: Figure out devnet config
        Self::mainnet()
    }
}

fn decode_aux_hash_from_base58(s: impl AsRef<[u8]>, check: u8) -> Vec<u8> {
    let bytes: Vec<u8> = bs58::decode(s)
        .with_check(Some(check))
        .into_vec()
        .expect(ERR_FAIL_TO_DECODE_B58);

    bytes.into_iter().skip(1).take(32).collect()
}
