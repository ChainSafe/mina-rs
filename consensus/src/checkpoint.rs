// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::common::{Common, ProtocolStateChain};
use mina_crypto::hash::{Hashable, StateHash};
use mina_rs_base::protocol_state::ProtocolState;

/// init_checkpoints initializes the checkpoints for the genesis block
/// This function assumes the state hash of `genesis` is already set
fn initCheckpoints(genesis: &mut ProtocolState) {
    let state_hash = genesis.hash();
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
        .next_epoch_data
        .start_checkpoint = state_hash;
    genesis.body.consensus_state.next_epoch_data.lock_checkpoint = state_hash;
}

fn updateCheckpoints(parent: &ProtocolState, block: &mut ProtocolState) {
    let parent_hash = parent.hash();
    if block.epoch_slot().unwrap() == 0 {
        block.body.consensus_state.next_epoch_data.start_checkpoint = parent_hash;
    }

    let epoch_slot = block.epoch_slot().unwrap();
    if epoch_slot < (2 / 3) * block.body.constants.slots_per_epoch.0 {
        block.body.consensus_state.next_epoch_data.lock_checkpoint = parent_hash;
    }
}

fn isShortRange (c0: &ProtocolStateChain, c1: &ProtocolStateChain) -> bool {
    c0.consensus_state()
        .unwrap()
        .staking_epoch_data
        .lock_checkpoint
        == c1
            .consensus_state()
            .unwrap()
            .staking_epoch_data
            .lock_checkpoint
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_rs_base::global_slot::GlobalSlot;
    use mina_rs_base::numbers::{GlobalSlotNumber, Length};

    #[test]
    fn test_init_checkpoints() {
        let mut genesis: ProtocolState = Default::default();
        let state_hash = genesis.hash();
        initCheckpoints(&mut genesis);
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
            state_hash
        );
        assert_eq!(
            genesis.body.consensus_state.next_epoch_data.lock_checkpoint,
            state_hash
        );
    }

    #[test]
    fn test_update_checkpoints() {
        let mut genesis: ProtocolState = Default::default();
        let state_hash = genesis.hash();
        initCheckpoints(&mut genesis);

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(1000),
        };
        updateCheckpoints(&genesis, &mut b1);
        assert_eq!(
            b1.body.consensus_state.next_epoch_data.start_checkpoint,
            state_hash
        );
        assert_eq!(
            b1.body.consensus_state.next_epoch_data.lock_checkpoint,
            state_hash
        );

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1),
            slots_per_epoch: Length(1000),
        };
        updateCheckpoints(&genesis, &mut b1);
        assert_eq!(
            b1.body.consensus_state.next_epoch_data.start_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            b1.body.consensus_state.next_epoch_data.lock_checkpoint,
            state_hash
        );

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(667),
            slots_per_epoch: Length(1000),
        };
        updateCheckpoints(&genesis, &mut b1);
        assert_eq!(
            b1.body.consensus_state.next_epoch_data.start_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            b1.body.consensus_state.next_epoch_data.lock_checkpoint,
            StateHash::default()
        );
    }
}
