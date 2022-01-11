// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use hex::ToHex;
use mina_crypto::hash::*;
use mina_rs_base::types::*;
use thiserror::Error;

pub struct ProtocolStateChain(pub Vec<ProtocolState>);

#[derive(Error, Debug, PartialEq)]
pub enum ChainError {
    #[error("header must have height 1 greater than top")]
    InvalidHeight,
}

pub trait Chain<T>
where
    T: Header,
{
    fn push(&mut self, new: T) -> Result<(), ChainError>;
    fn top(&self) -> Option<&T>;
    fn consensus_state(&self) -> Option<&ConsensusState>;
    fn global_slot(&self) -> Option<&GlobalSlot>;
    fn epoch_slot(&self) -> Option<u32>;
    fn length(&self) -> usize;
    fn last_vrf(&self) -> Option<String>;
    fn state_hash(&self) -> Option<StateHash>;
}

impl Chain<ProtocolState> for ProtocolStateChain {
    fn push(&mut self, new: ProtocolState) -> Result<(), ChainError> {
        match self.0.len() {
            0 => (),
            n => {
                if new.get_height().0 != self.0[n - 1].get_height().0 + 1 {
                    return Err(ChainError::InvalidHeight);
                }
            }
        }

        self.0.push(new);
        Ok(())
    }

    fn top(&self) -> Option<&ProtocolState> {
        self.0.last()
    }

    fn consensus_state(&self) -> Option<&ConsensusState> {
        self.top().map(|s| &s.body.consensus_state)
    }

    fn global_slot(&self) -> Option<&GlobalSlot> {
        self.top().map(|s| &s.body.consensus_state.curr_global_slot)
    }

    fn epoch_slot(&self) -> Option<u32> {
        self.global_slot()
            .map(|s| (s.slot_number.0 % s.slots_per_epoch.0))
    }

    fn length(&self) -> usize {
        self.0.len()
    }

    fn last_vrf(&self) -> Option<String> {
        self.top().map(|s| {
            s.body
                .consensus_state
                .last_vrf_output
                .hash()
                .as_ref()
                .encode_hex::<String>()
        })
    }

    fn state_hash(&self) -> Option<StateHash> {
        self.top().map(|s| s.hash())
    }
}

pub fn gen_spot(block: &mut ProtocolState) {
    // New default consensus state
    // Generate blockchain position and epoch lengths.
    // staking_epoch == root_epoch, next_staking_epoch == root_epoch + 1
    // New state slot and length.
    // New total currency for state.
    // New epoch data for staking and next epochs.
    // New block reward information (unused in chain selection).
    block.body.consensus_state = ConsensusState::default();
}
