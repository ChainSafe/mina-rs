// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::common::{Chain, ProtocolStateChain};
use mina_crypto::base58::Base58Encodable;
use mina_crypto::hash::{EpochSeed, StateHash};
use mina_rs_base::protocol_state::ProtocolState;

pub const SLOTS_PER_EPOCH: u32 = 7140;

#[derive(Debug)]
pub enum ConsensusErrTyp {
    ConsensusInitFail,
}
pub fn init_checkpoints(genesis: &mut ProtocolState) -> Result<(), ConsensusErrTyp> {
    genesis.body.consensus_state.staking_epoch_data.seed = EpochSeed::default();
    genesis
        .body
        .consensus_state
        .staking_epoch_data
        .start_checkpoint = StateHash::default();
    genesis
        .body
        .consensus_state
        .staking_epoch_data
        .lock_checkpoint = StateHash::default();
    genesis
        .body
        .consensus_state
        .staking_epoch_data
        .epoch_length
        .0 = 1;
    genesis.body.consensus_state.next_epoch_data.seed =
        Base58Encodable::from_base58("2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt")
            .map_err(|_| ConsensusErrTyp::ConsensusInitFail)?;
    genesis
        .body
        .consensus_state
        .next_epoch_data
        .start_checkpoint = StateHash::default();
    genesis.body.consensus_state.next_epoch_data.lock_checkpoint =
        Base58Encodable::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
            .map_err(|_| ConsensusErrTyp::ConsensusInitFail)?;
    genesis.body.consensus_state.next_epoch_data.epoch_length.0 = 2;
    Ok(())
}

pub fn is_short_range(
    c0: &ProtocolStateChain,
    c1: &ProtocolStateChain,
) -> Result<bool, ConsensusErrTyp> {
    let s0 = &c0
        .consensus_state()
        .ok_or(ConsensusErrTyp::ConsensusInitFail)?;
    let s1 = &c1
        .consensus_state()
        .ok_or(ConsensusErrTyp::ConsensusInitFail)?;
    let s0_lock_checkpoint = &s0.staking_epoch_data.lock_checkpoint;
    let s1_lock_checkpoint = &s1.staking_epoch_data.lock_checkpoint;
    let s1_next_epoch_lock_checkpoint = &s1.next_epoch_data.lock_checkpoint;

    if s0.epoch_count == s1.epoch_count {
        return Ok(s0_lock_checkpoint == s1_lock_checkpoint);
    }

    if s0.epoch_count.0 == s1.epoch_count.0 + 1 && c1.epoch_slot() >= Some(SLOTS_PER_EPOCH * 2 / 3)
    {
        Ok(s0_lock_checkpoint == s1_next_epoch_lock_checkpoint)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_rs_base::global_slot::GlobalSlot;
    use mina_rs_base::numbers::{GlobalSlotNumber, Length};
    use mina_rs_base::{consensus_state::ConsensusState, protocol_state::ProtocolConstants};
    extern crate quickcheck;
    use proptest::prelude::*;
    use quickcheck::QuickCheck;
    use wasm_bindgen_test::*;

    #[test]
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
            Base58Encodable::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
                .unwrap()
        );
    }

    fn gen_spot_root_epoch_position(slot_fill_rate: f64, slot_fill_rate_delta: f64) {
        //   TODO: We need to simulate both the staking epoch and the next staking epoch,
        //  the root epoch is the staking epoch. The root epoch position this function generates
        //   is the epoch number of the staking epoch and the block height the
        //  staking epoch starts at (the simulation of all blocks preceeding the
        //  staking epoch
    }

    fn gen_spot(block: &mut ProtocolState) {
        // New default consensus state and Protocol constant
        // Generate blockchain position and epoch lengths.
        // staking_epoch == root_epoch, next_staking_epoch == root_epoch + 1
        // TODO: Compute state slot and length.
        // Compute total currency for state.
        // Generate epoch data for staking and next epochs.
        // TODO: Generate chain quality and vrf output.
        // Generate block reward information (unused in chain selection).
        let consensus_state = ConsensusState::new();
        let protocol_constants = ProtocolConstants::new();
        block.body.consensus_state = consensus_state;
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

        let mut c0: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c1: ProtocolStateChain = ProtocolStateChain(vec![]);

        c0.push(a).unwrap();
        c1.push(b).unwrap();
        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
        assert_eq!(is_short_range(&c1, &c0).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn gen_spot_pair_short_aligned_generates_pairs_of_states_in_short_fork_range() {
        // TODO: Both states will share their staking epoch checkpoints.
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        gen_spot(&mut a);
        gen_spot(&mut b);

        let mut c0: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c1: ProtocolStateChain = ProtocolStateChain(vec![]);

        c0.push(a).unwrap();
        c1.push(b).unwrap();
        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
        assert_eq!(is_short_range(&c1, &c0).unwrap(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn gen_spot_pair_short_misaligned_generates_pairs_of_states_in_short_fork_range() {
        // TODO: Compute the root epoch position of `b`. This needs to be one epoch ahead of a, so we
        // compute it by extending the root epoch position of `a` by a single epoch
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();

        let mut a: ProtocolState = Default::default();
        let mut b: ProtocolState = Default::default();

        gen_spot(&mut a);
        gen_spot(&mut b);

        let mut c0: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c1: ProtocolStateChain = ProtocolStateChain(vec![]);

        // TODO Constrain first state to be within last 1/3rd of its epoch (ensuring it's checkpoints and seed are fixed). *)
        // let min_a_curr_epoch_slot = (2 * (Length.to_int constants.slots_per_epoch / 3)) + 1;

        c0.push(a).unwrap();
        c1.push(b).unwrap();
        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
        assert_eq!(is_short_range(&c1, &c0).unwrap(), true);
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
        let mut c0: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c1: ProtocolStateChain = ProtocolStateChain(vec![]);

        c0.push(a).unwrap();
        c1.push(b).unwrap();
        assert_eq!(is_short_range(&c0, &c1).unwrap(), false);
        assert_eq!(is_short_range(&c1, &c0).unwrap(), false);
    }
}
