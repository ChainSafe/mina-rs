// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::common::{Chain, ProtocolStateChain};
use crate::constants::SLOTS_PER_EPOCH;
use crate::error::ConsensusError;

pub fn is_short_range(
    c0: &ProtocolStateChain,
    c1: &ProtocolStateChain,
) -> Result<bool, ConsensusError> {
    let s0 = &c0
        .consensus_state()
        .ok_or(ConsensusError::ConsensusStateNotFound)?;
    let s1 = &c1
        .consensus_state()
        .ok_or(ConsensusError::ConsensusStateNotFound)?;
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
