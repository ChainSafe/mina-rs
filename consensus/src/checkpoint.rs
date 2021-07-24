use crate::common::Chain;
use mina_rs_base::protocol_state::{Header, ProtocolState};

/// init_checkpoints initializes the checkpoints for the genesis block
fn init_checkpoints(g: ProtocolState) -> () {
    ()
}

fn update_checkpounts(parent: ProtocolState, block: ProtocolState) -> () {
    ()
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
