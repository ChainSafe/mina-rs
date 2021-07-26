use crate::common::Chain;
use mina_crypto::hash::{Hashable, EMPTY_STATE_HASH};
use mina_rs_base::protocol_state::ProtocolState;

/// init_checkpoints initializes the checkpoints for the genesis block
/// This function assumes the state hash of `g` is already set
fn init_checkpoints(mut g: ProtocolState) -> () {
    let state_hash = g.hash();
    g.body.consensus_state.staking_epoch_data.start_checkpoint = EMPTY_STATE_HASH;
    g.body.consensus_state.staking_epoch_data.lock_checkpoint = EMPTY_STATE_HASH;
    g.body.consensus_state.next_epoch_data.start_checkpoint = state_hash;
    g.body.consensus_state.next_epoch_data.lock_checkpoint = state_hash;
}

fn update_checkpounts(parent: ProtocolState, mut block: ProtocolState) -> () {
    let parent_hash = parent.hash();
    if block.epoch_slot().unwrap() == 0 {
        block.body.consensus_state.next_epoch_data.start_checkpoint = parent_hash;
    }

    let epoch_slot = block.epoch_slot().unwrap();
    if 0 <= epoch_slot && epoch_slot < (2 / 3) * block.body.constants.slots_per_epoch.0 {
        block.body.consensus_state.next_epoch_data.lock_checkpoint = parent_hash;
    }
}

fn is_short_range(c0: &dyn Chain<ProtocolState>, c1: &dyn Chain<ProtocolState>) -> bool {
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
