// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

impl Genesis for ExternalTransition {
    /// Initialize genesis block
    /// <https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#611-genesis-block>
    fn from_genesis_config(config: &GenesisInitConfig) -> ExternalTransition {
        let protocol_state = ProtocolState {
            body: ProtocolStateBody {
                blockchain_state: config.blockchain_state.clone(),
                constants: config.constants.clone(),
                consensus_state: ConsensusState {
                    block_creator: config.block_creator,
                    block_stake_winner: config.block_stake_winner,
                    blockchain_length: 1_u32.into(),
                    epoch_count: 0_u32.into(),
                    coinbase_receiver: config.coinbase_receiver,
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
