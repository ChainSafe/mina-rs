// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_consensus::{checkpoint::*, common::*};
    use mina_crypto::{hash::*, prelude::*};
    use mina_rs_base::numbers::Length;
    use mina_rs_base::protocol_state::ProtocolConstantsTemp;
    use mina_rs_base::types::*;
    use rand::{thread_rng, Rng};
    use wasm_bindgen_test::*;
    extern crate quickcheck;
    use proptest::prelude::*;
    use quickcheck::QuickCheck;
    use wasm_bindgen_test::*;
    const DEFAULT_SLOT_FILL_RATE: f64 = 0.65;
    const DEFAULT_SLOT_FILL_RATE_DELTA: f64 = 0.15;

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

    fn gen_num_blocks_in_epochs(slot_fill_rate: f64, slot_fill_rate_delta: f64, n: f64) -> i32 {
        let protocol_constants = ProtocolConstantsTemp::default();
        let nums = gen_num_blocks_in_slots(
            slot_fill_rate,
            slot_fill_rate_delta,
            n * protocol_constants.slots_per_epoch.0 as f64,
        );
        nums
    }

    fn gen_spot_root_epoch_position(slot_fill_rate: f64, slot_fill_rate_delta: f64) -> (u32, u32) {
        //  We need to simulate both the staking epoch and the next staking epoch,
        //  the root epoch is the staking epoch. The root epoch position this function generates
        //   is the epoch number of the staking epoch and the block height the
        //  staking epoch starts at (the simulation of all blocks preceeding the
        //  staking epoch
        let root_epoch_int = thread_rng().gen_range(0..100);
        let root_block_height =
            gen_num_blocks_in_epochs(slot_fill_rate, slot_fill_rate_delta, root_epoch_int as f64);
        return (root_epoch_int, root_block_height as u32);
    }

    fn convert(x: f64) -> i32 {
        x.round().rem_euclid(2f64.powi(32)) as u32 as i32
    }

    fn gen_num_blocks_in_slots(slot_fill_rate: f64, slot_fill_rate_delta: f64, n: f64) -> i32 {
        let min_blocks = n * f64::max(slot_fill_rate - slot_fill_rate_delta, 0.0);
        let max_blocks = n * f64::min(slot_fill_rate + slot_fill_rate_delta, 1.0);
        let num_blocks_in_slots = thread_rng().gen_range(min_blocks..max_blocks);
        let num = convert(num_blocks_in_slots);
        return num;
    }

    fn gen_spot_pair_common_checkpoints(
        a: &mut ProtocolState,
        b: &mut ProtocolState,
        min_a_curr_epoch_slot: u32,
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
        let _blockchain_length_relativity = Some(&a);
        let max_epoch_slot = match _blockchain_length_relativity {
            Some(_blockchain_length_relativity) => {
                a.body.consensus_state.curr_global_slot.slots_per_epoch.0 - 4
            } // -1 to bring into inclusive range, -3 to provide 2 slots of fudge room
            None => a.body.consensus_state.curr_global_slot.slots_per_epoch.0 - 1, // -1 to bring into inclusive range
        };

        let min_a_curr_epoch_slot_defaut = 0;
        let min_a_curr_epoch_slot_sum = min_a_curr_epoch_slot_defaut + min_a_curr_epoch_slot;
        let slot = thread_rng().gen_range(min_a_curr_epoch_slot_sum..max_epoch_slot);
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
        let protocol_constants = ProtocolConstantsTemp::default();
        let max_epoch_slot = protocol_constants.slots_per_epoch.0 - 1;

        // This invariant needs to be held for the position of `a`
        assert!(max_epoch_slot > a_curr_epoch_slot.0 + 2);

        // Assume there is a next block in the slot directly preceeding the block for `a`
        let added_slots = thread_rng().gen_range(a_curr_epoch_slot.0 + 2..max_epoch_slot);

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
    fn equal_state_in_short_fork_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        gen_spot(&mut a);
        gen_spot(&mut b);

        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn gen_spot_pair_short_aligned_generates_pairs_of_states_in_short_fork_range() {
        // Both states will share their staking epoch checkpoints.
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        gen_spot(&mut a);
        gen_spot(&mut b);
        gen_spot_pair_common_checkpoints(&mut a, &mut b, 0);

        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn gen_spot_pair_short_misaligned_generates_pairs_of_states_in_short_fork_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        gen_spot(&mut a);
        gen_spot(&mut b);

        // Compute the root epoch position of `b`. This needs to be one epoch ahead of a
        let added_blocks =
            gen_num_blocks_in_slots(DEFAULT_SLOT_FILL_RATE, DEFAULT_SLOT_FILL_RATE_DELTA, 1.0);

        b.body.consensus_state.blockchain_length.0 =
            a.body.consensus_state.blockchain_length.0 + added_blocks as u32;

        // Constrain first state to be within last 1/3rd of its epoch (ensuring it's checkpoints and seed are fixed)
        let protocol_constants = ProtocolConstantsTemp::default();
        let min_a_curr_epoch_slot = 2 * (protocol_constants.slots_per_epoch.0 / 3) + 1;
        gen_spot_pair_common_checkpoints(&mut a, &mut b, min_a_curr_epoch_slot);

        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn gen_spot_pair_long_generates_pairs_of_states_in_long_fork_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        gen_spot(&mut a);
        gen_spot(&mut b);
        a.body.consensus_state.epoch_count = Length(14);
        b.body.consensus_state.epoch_count = Length(15);
        let c0: ProtocolStateChain = ProtocolStateChain(vec![a]);
        let c1: ProtocolStateChain = ProtocolStateChain(vec![b]);

        assert_eq!(is_short_range(&c0, &c1).unwrap(), false);
    }
}
