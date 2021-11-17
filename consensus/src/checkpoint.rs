// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::common::{Common, ProtocolStateChain};
use mina_crypto::base58::Base58Encodable;
use mina_crypto::hash::{EpochSeed, StateHash};
use mina_rs_base::protocol_state::ProtocolState;
use wasm_bindgen_test::*;
const SLOTS_PER_EPOCH: u32 = 7140;

/// init_checkpoints initializes the checkpoints for the genesis block
/// This function assumes the state hash of `genesis` is already set
fn init_checkpoints(genesis: &mut ProtocolState) {
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
            .unwrap();
    genesis
        .body
        .consensus_state
        .next_epoch_data
        .start_checkpoint = StateHash::default();
    genesis.body.consensus_state.next_epoch_data.lock_checkpoint =
        Base58Encodable::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
            .unwrap();
    genesis.body.consensus_state.next_epoch_data.epoch_length.0 = 2;
}

fn is_short_range(c0: &ProtocolStateChain, c1: &ProtocolStateChain) -> bool {
    if c0.consensus_state().unwrap().epoch_count == c1.consensus_state().unwrap().epoch_count {
        return c0
            .consensus_state()
            .unwrap()
            .staking_epoch_data
            .lock_checkpoint
            == c1
                .consensus_state()
                .unwrap()
                .staking_epoch_data
                .lock_checkpoint;
    }

    if c0.consensus_state().unwrap().epoch_count.0
        == c1.consensus_state().unwrap().epoch_count.0 + 1
        && Common::epoch_slot(c1) >= Some((2 / 3) * SLOTS_PER_EPOCH)
    {
        return c0
            .consensus_state()
            .unwrap()
            .staking_epoch_data
            .lock_checkpoint
            == c1
                .consensus_state()
                .unwrap()
                .next_epoch_data
                .lock_checkpoint;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_rs_base::global_slot::GlobalSlot;
    use mina_rs_base::numbers::{GlobalSlotNumber, Length};

    #[wasm_bindgen_test]
    fn test_init_checkpoints() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis);
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

    #[wasm_bindgen_test]
    fn test_is_short_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis);
        let mut c0: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c1: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        b0.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1),
            slots_per_epoch: Length(7140),
        };
        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(2),
            slots_per_epoch: Length(7140),
        };

        c0.push(b0.clone()).unwrap();
        c1.push(b1.clone()).unwrap();
        assert_eq!(is_short_range(&c0, &c1), true);
    }
}
