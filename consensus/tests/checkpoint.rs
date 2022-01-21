// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_consensus::{checkpoint::*, common::*};
    use mina_crypto::{hash::*, prelude::*};
    use mina_rs_base::numbers::Length;
    use mina_rs_base::types::*;
    use wasm_bindgen_test::*;
    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L3948
    /// The default slot fill rate
    const DEFAULT_SLOT_FILL_RATE: f64 = 0.65;
    /// The changes of default slot fill rate
    const DEFAULT_SLOT_FILL_RATE_DELTA: f64 = 0.15;
    /// Define constant for 2f64.powi(32)
    const TRANS: f64 = 4294967296_f64;

    #[test]
    #[wasm_bindgen_test]
    fn test_init_checkpoints() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();
        assert_eq!(
            genesis
                .body
                .consensus_state
                .staking_epoch_data
                .start_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            genesis
                .body
                .consensus_state
                .staking_epoch_data
                .lock_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            genesis
                .body
                .consensus_state
                .next_epoch_data
                .start_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            genesis.body.consensus_state.next_epoch_data.lock_checkpoint,
            StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d").unwrap()
        );
    }

    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L3881
    fn gen_num_blocks_in_epochs(slot_fill_rate: f64, slot_fill_rate_delta: f64, n: f64) -> i32 {
        let protocol_constants = ConsensusConstants::default();
        let nums = gen_num_blocks_in_slots(
            slot_fill_rate,
            slot_fill_rate_delta,
            n * protocol_constants.slots_per_epoch.0 as f64,
        );
        nums
    }

    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L3960
    fn gen_spot_root_epoch_position(slot_fill_rate: f64, slot_fill_rate_delta: f64) -> (u32, u32) {
        //  We need to simulate both the staking epoch and the next staking epoch,
        //  the root epoch is the staking epoch. The root epoch position this function generates
        //   is the epoch number of the staking epoch and the block height the
        //  staking epoch starts at (the simulation of all blocks preceeding the
        //  staking epoch
        let root_epoch_int = 50;
        let root_block_height =
            gen_num_blocks_in_epochs(slot_fill_rate, slot_fill_rate_delta, root_epoch_int as f64);
        return (root_epoch_int, root_block_height as u32);
    }

    fn convert(x: f64) -> i32 {
        x.round().rem_euclid(TRANS) as u32 as i32
    }

    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L3867
    fn gen_num_blocks_in_slots(slot_fill_rate: f64, slot_fill_rate_delta: f64, n: f64) -> i32 {
        let min_blocks = n * f64::max(slot_fill_rate - slot_fill_rate_delta, 0.0);
        let max_blocks = n * f64::min(slot_fill_rate + slot_fill_rate_delta, 1.0);
        let num_blocks_in_slots = (min_blocks + max_blocks) / 2.0;
        let num = convert(num_blocks_in_slots);
        return num;
    }

    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L4111
    fn gen_spot_pair_common_checkpoints(
        a: &mut ProtocolState,
        b: &mut ProtocolState,
        min_a_curr_epoch_slot: u32,
        _blockchain_length_relativity: String,
    ) {
        // New pairs of spot blocks that share common checkpoints.
        // The overlap of the checkpoints and the root epoch positions of the blocks
        // that are generated can be configured independently so that this function
        // can be used in other generators that wish to generates pairs of spot blocks
        // with specific constraints.
        let base_root_epoch_position =
            gen_spot_root_epoch_position(DEFAULT_SLOT_FILL_RATE, DEFAULT_SLOT_FILL_RATE_DELTA);

        // Constraining the second state to have a greater blockchain length than the
        // first, we need to constrain the first blockchain length such that there is some room
        // leftover in the epoch for at least 1 more block to be generated.
        let max_epoch_slot = match &_blockchain_length_relativity[..] {
            // -1 to bring into inclusive range, -3 to provide 2 slots of fudge room
            "Ascending" => a.body.consensus_state.curr_global_slot.slots_per_epoch.0 - 4,
            // -1 to bring into inclusive range
            _ => a.body.consensus_state.curr_global_slot.slots_per_epoch.0 - 1,
        };

        let min_a_curr_epoch_slot_defaut = 0;
        let min_a_curr_epoch_slot_sum = min_a_curr_epoch_slot_defaut + min_a_curr_epoch_slot;
        let slot = (min_a_curr_epoch_slot_sum + max_epoch_slot) / 2;
        let length = gen_num_blocks_in_slots(
            DEFAULT_SLOT_FILL_RATE,
            DEFAULT_SLOT_FILL_RATE_DELTA,
            slot as f64,
        );

        a.body.consensus_state.curr_global_slot.slot_number.0 = slot;
        a.body.consensus_state.blockchain_length.0 = length as u32;

        // Randomized root_epoch_position for more robust test
        let root_epoch_position = base_root_epoch_position;
        let (_, root_epoch_length) = root_epoch_position;

        let length_till_curr_epoch = root_epoch_length
            + a.body.consensus_state.staking_epoch_data.epoch_length.0
            + a.body.consensus_state.next_epoch_data.epoch_length.0;
        let a_curr_epoch_length = length_till_curr_epoch;

        // Handle relativity constriants for second state.
        let a_curr_epoch_slot = &a.body.consensus_state.curr_global_slot.slot_number;

        // Generate second state position by extending the first state's position
        let protocol_constants = ConsensusConstants::default();
        let max_epoch_slot = protocol_constants.slots_per_epoch.0 - 1;

        // This invariant needs to be held for the position of `a`
        assert!(max_epoch_slot > a_curr_epoch_slot.0 + 2);

        // Assume mix ascending there is a next block in the slot directly preceeding the block for `a`
        let added_slots = (a_curr_epoch_slot.0 + 2 + max_epoch_slot) / 2;

        let added_blocks = gen_num_blocks_in_slots(
            DEFAULT_SLOT_FILL_RATE,
            DEFAULT_SLOT_FILL_RATE_DELTA,
            added_slots as f64,
        );

        b.body.consensus_state.curr_global_slot.slot_number.0 =
            a_curr_epoch_slot.0 + added_slots + 1;
        b.body.consensus_state.blockchain_length.0 = a_curr_epoch_length + added_blocks as u32 + 1;
    }

    #[test]
    #[wasm_bindgen_test]
    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L4394    
    fn equal_state_in_short_fork_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        a.body.consensus_state = ConsensusState::default();
        b.body.consensus_state = ConsensusState::default();

        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L4258
    fn gen_spot_pair_short_aligned_generates_pairs_of_states_in_short_fork_range() {
        // Both states will share their staking epoch checkpoints.
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        a.body.consensus_state = ConsensusState::default();
        b.body.consensus_state = ConsensusState::default();
        gen_spot_pair_common_checkpoints(&mut a, &mut b, 0, "None".to_string());

        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L4266
    fn gen_spot_pair_short_misaligned_generates_pairs_of_states_in_short_fork_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        a.body.consensus_state = ConsensusState::default();
        b.body.consensus_state = ConsensusState::default();

        // Compute the root epoch position of `b`. This needs to be one epoch ahead of a
        let added_blocks =
            gen_num_blocks_in_slots(DEFAULT_SLOT_FILL_RATE, DEFAULT_SLOT_FILL_RATE_DELTA, 1.0);

        b.body.consensus_state.blockchain_length.0 =
            a.body.consensus_state.blockchain_length.0 + added_blocks as u32;

        // Constrain first state to be within last 1/3rd of its epoch (ensuring it's checkpoints and seed are fixed)
        let protocol_constants = ConsensusConstants::default();
        let min_a_curr_epoch_slot = 2 * (protocol_constants.slots_per_epoch.0 / 3) + 1;
        gen_spot_pair_common_checkpoints(&mut a, &mut b, min_a_curr_epoch_slot, "None".to_string());

        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]

    /// https://github.com/MinaProtocol/mina/blob/af76cb7980d5e81e704120290a850ea9c6f8522e/src/lib/consensus/proof_of_stake.ml#L4291
    fn gen_spot_pair_long_generates_pairs_of_states_in_long_fork_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        a.body.consensus_state = ConsensusState::default();
        b.body.consensus_state = ConsensusState::default();

        a.body.consensus_state.epoch_count = Length(14);
        b.body.consensus_state.epoch_count = Length(15);
        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), false);
    }
}
