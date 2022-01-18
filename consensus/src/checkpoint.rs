// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::common::{Chain, ProtocolStateChain};
use crate::constants::SLOTS_PER_EPOCH;
use mina_crypto::base58::Base58Encodable;
use mina_crypto::hash::{EpochSeed, StateHash};
use mina_rs_base::protocol_state::ProtocolState;

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
        EpochSeed::from_base58("2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt")
            .map_err(|_| ConsensusErrTyp::ConsensusInitFail)?;
    genesis
        .body
        .consensus_state
        .next_epoch_data
        .start_checkpoint = StateHash::default();
    genesis.body.consensus_state.next_epoch_data.lock_checkpoint =
        StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
            .map_err(|_| ConsensusErrTyp::ConsensusInitFail)?;
    genesis.body.consensus_state.next_epoch_data.epoch_length.0 = 2;
    Ok(())
}

/// A fork is considered short-range if either
/// the fork point of the candidate chains are in the same epoch
/// or the fork point is in the previous epoch with the same lock_checkpoint
pub fn is_short_range(
    c0: &ProtocolStateChain,
    c1: &ProtocolStateChain,
) -> Result<bool, ConsensusErrTyp> {
    // Get consensus state from top blocks of each chain
    let s0 = &c0
        .consensus_state()
        .ok_or(ConsensusErrTyp::ConsensusInitFail)?;
    let s1 = &c1
        .consensus_state()
        .ok_or(ConsensusErrTyp::ConsensusInitFail)?;
    let s0_lock_checkpoint = &s0.staking_epoch_data.lock_checkpoint;
    let s1_lock_checkpoint = &s1.staking_epoch_data.lock_checkpoint;
    let s1_next_epoch_lock_checkpoint = &s1.next_epoch_data.lock_checkpoint;
    let check = |s0_lock_checkpoint, s1_next_epoch_lock_checkpoint| {
        if s0.epoch_count.0 == s1.epoch_count.0 + 1
            && c1.epoch_slot() >= Some(SLOTS_PER_EPOCH * 2 / 3)
        {
            // S1 is one epoch ahead of S2 and S2 is not in the seed update range
            s0_lock_checkpoint == s1_next_epoch_lock_checkpoint
        } else {
            false
        }
    };

    if s0.epoch_count == s1.epoch_count {
        // Simple case: blocks have same previous epoch, so compare previous epochs' lock_checkpoints
        Ok(s0_lock_checkpoint == s1_lock_checkpoint)
    } else {
        // Check for previous epoch case using both orientations
        Ok(check(s0_lock_checkpoint, s1_next_epoch_lock_checkpoint)
            || check(s1_next_epoch_lock_checkpoint, s0_lock_checkpoint))
    }
}
