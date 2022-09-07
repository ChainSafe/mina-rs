// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use mina_crypto::hash::*;
use once_cell::sync::OnceCell;
use proof_systems::mina_signer::CompressedPubKey;
use std::str::FromStr;

impl Genesis for ExternalTransition {
    /// Initialize genesis block
    /// <https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#611-genesis-block>
    fn from_genesis_config(config: &GenesisInitConfig) -> ExternalTransition {
        let protocol_state = ProtocolStateLegacy {
            body: ProtocolStateBodyLegacy {
                blockchain_state: config.blockchain_state.clone(),
                constants: config.constants.clone(),
                consensus_state: ConsensusState {
                    block_creator: config.block_creator.clone(),
                    block_stake_winner: config.block_stake_winner.clone(),
                    blockchain_length: 1_u32.into(),
                    epoch_count: 0_u32.into(),
                    coinbase_receiver: config.coinbase_receiver.clone(),
                    curr_global_slot: GlobalSlot {
                        slot_number: 0_u32.into(),
                        slots_per_epoch: config.constants.slots_per_epoch,
                    },
                    global_slot_since_genesis: 0_u32.into(),
                    has_ancestor_in_same_checkpoint_window: true,
                    last_vrf_output: config.last_vrf_output.clone(),
                    min_window_density: (config.sub_windows_per_window
                        * config.constants.slots_per_sub_window.0)
                        .into(),
                    next_epoch_data: config.next_epoch_data.clone(),
                    staking_epoch_data: config.staking_epoch_data.clone(),
                    sub_window_densities: config.sub_window_densities.clone(),
                    supercharge_coinbase: true,
                    total_currency: config.total_currency,
                },
                genesis_state_hash: config.genesis_state_hash.clone(),
            },
            previous_state_hash: config.previous_state_hash.clone(),
        };

        ExternalTransition {
            staged_ledger_diff: StagedLedgerDiff::default(),
            protocol_state,
            protocol_state_proof: config.protocol_state_proof.clone(),
            delta_transition_chain_proof: config.delta_transition_chain_proof.clone(),
            current_protocol_version: ProtocolVersion::default(),
            proposed_protocol_version_opt: None,
            validation_callback: (),
        }
    }
}

impl GenesisProtocolState<ProtocolState> for ProtocolState {
    fn berkeley() -> &'static ProtocolState {
        fn gen() -> anyhow::Result<ProtocolState> {
            Ok(ProtocolState {
                previous_state_hash: StateHash::from_str(
                    "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
                )?,
                body: ProtocolStateBody {
                    genesis_state_hash: StateHash::from_str(
                        "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
                    )?,
                    blockchain_state: BlockchainState {
                        staged_ledger_hash: StagedLedgerHash {
                            non_snark: NonSnarkStagedLedgerHash {
                                ledger_hash: LedgerHash::from_str(
                                    "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                                )?,
                                aux_hash: AuxHash::from_str(
                                    "UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom",
                                )?,
                                pending_coinbase_aux: PendingCoinbaseAuxHash::from_str(
                                    "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
                                )?,
                            },
                            pending_coinbase_hash: CoinBaseHash::from_str(
                                "2n27mUhCEctJbiZQdrk3kxYc7DVHvJVDErjXrjNs7jnP3HMLKtuN",
                            )?,
                        },
                        genesis_ledger_hash: LedgerHash::from_str(
                            "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                        )?,
                        registers: BlockchainStateRegisters {
                            ledger: LedgerHash::from_str(
                                "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                            )?,
                            pending_coinbase_stack: (),
                            local_state: Default::default(),
                        },
                        timestamp: BlockTime(1655755201000),
                        body_reference: BodyReference::from_hex(
                            "36bda176656cc3be96c3d317db7b4ac06fdbc7f4eedcd6efdd20e28143d67421",
                        )?,
                    },
                    consensus_state: ConsensusState {
                        blockchain_length: 1.into(),
                        epoch_count: 0.into(),
                        min_window_density: 77.into(),
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
                        last_vrf_output: VrfOutputTruncated::from_str(
                            "OruOTtGM3tJL3jM0GHtCzKyugvWT0ZP7VckspHX8_g8=",
                        )?,
                        total_currency: 1013238001000001000.into(),
                        curr_global_slot: GlobalSlot {
                            slot_number: 0.into(),
                            slots_per_epoch: 7140.into(),
                        },
                        global_slot_since_genesis: 0.into(),
                        staking_epoch_data: EpochData {
                            ledger: EpochLedger {
                                hash: LedgerHash::from_str(
                                    "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                                )?,
                                total_currency: 1013238001000001000.into(),
                            },
                            seed: EpochSeed::from_str(
                                "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA",
                            )?,
                            start_checkpoint: StateHash::from_str(
                                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                            )?,
                            lock_checkpoint: StateHash::from_str(
                                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                            )?,
                            epoch_length: 1.into(),
                        },
                        next_epoch_data: EpochData {
                            ledger: EpochLedger {
                                hash: LedgerHash::from_str(
                                    "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                                )?,
                                total_currency: 1013238001000001000.into(),
                            },
                            seed: EpochSeed::from_str(
                                "2vc1zQHJx2xN72vaR4YDH31KwFSr5WHSEH2dzcfcq8jxBPcGiJJA",
                            )?,
                            start_checkpoint: StateHash::from_str(
                                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                            )?,
                            lock_checkpoint: StateHash::from_str(
                                "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
                            )?,
                            epoch_length: 2.into(),
                        },
                        has_ancestor_in_same_checkpoint_window: true,
                        block_stake_winner: CompressedPubKey::from_address(
                            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                        )?,
                        block_creator: CompressedPubKey::from_address(
                            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                        )?,
                        coinbase_receiver: CompressedPubKey::from_address(
                            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                        )?,
                        supercharge_coinbase: true,
                    },
                    constants: Default::default(),
                },
            })
        }

        static INSTANCE: OnceCell<ProtocolState> = OnceCell::new();
        INSTANCE.get_or_init(|| gen().expect("Failed to initialize berkeley protocol state"))
    }
}
