// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_rs_base::network_types::ExternalTransitionV1;

use super::*;

impl Genesis for ExternalTransition {
    /// Initialize genesis block
    /// <https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#611-genesis-block>
    fn from_genesis_config(config: &GenesisInitConfig) -> Self {
        let mut et = ExternalTransition::default();

        et.protocol_state.body.blockchain_state = config.blockchain_state.clone();
        et.protocol_state.body.constants = config.constants.clone();

        let cs = &mut et.protocol_state.body.consensus_state;
        cs.blockchain_length = 1.into();
        cs.epoch_count = 0.into();
        cs.min_window_density =
            (config.sub_windows_per_window * config.constants.slots_per_sub_window.0).into();
        cs.sub_window_densities = config.sub_window_densities.clone();
        cs.last_vrf_output = config.last_vrf_output.clone();
        cs.total_currency = config.total_currency;
        cs.curr_global_slot = GlobalSlot {
            slot_number: 0.into(),
            slots_per_epoch: config.constants.slots_per_epoch,
        };
        cs.global_slot_since_genesis = 0.into();
        cs.staking_epoch_data = config.staking_epoch_data.clone();
        cs.next_epoch_data = config.next_epoch_data.clone();
        cs.has_ancestor_in_same_checkpoint_window = true;
        cs.block_stake_winner = config.block_stake_winner.clone();
        cs.block_creator = config.block_creator.clone();
        cs.coinbase_receiver = config.coinbase_receiver.clone();
        cs.supercharge_coinbase = true;

        et.protocol_state.previous_state_hash = config.previous_state_hash.clone();
        et.protocol_state.body.genesis_state_hash = config.genesis_state_hash.clone();

        et.protocol_state_proof = config.protocol_state_proof.clone();

        et.delta_transition_chain_proof = config.delta_transition_chain_proof.clone();
        et.current_protocol_version = ProtocolVersion::default();
        et.proposed_protocol_version_opt = None;

        et
    }
}

impl Genesis for ExternalTransitionV1 {
    fn from_genesis_config(config: &GenesisInitConfig) -> Self {
        ExternalTransition::from_genesis_config(config).into()
    }
}
