// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Implements common APIs for the blockchain in the context of consensus.

use hex::ToHex;
use mina_crypto::hash::{Hashable, StateHash};
use mina_rs_base::consensus_state::ConsensusState;
use mina_rs_base::global_slot::GlobalSlot;
use mina_rs_base::protocol_state::{Header, ProtocolState};

use crate::density::{relative_min_window_density, ConsensusConstants};
use crate::error::ConsensusError;

#[derive(Clone, Debug, Default)] // FIXME: remove clone here.
                                 // TODO: replace vec element with ExternalTransition
pub struct ProtocolStateChain(pub Vec<ProtocolState>);

pub trait Chain<T>
where
    T: Header,
{
    fn push(&mut self, new: T) -> Result<(), ConsensusError>;
    /// This function returns the last block of a given chain.
    /// The input is a chain C and the output is last block of C
    /// (i.e. the block with greatest height).
    fn top(&self) -> Option<&T>;
    /// he function returns the consensus state of a block or chain.
    /// The input is a block or chain X and the output is the consensus state.
    fn consensus_state(&self) -> Option<&ConsensusState>;
    /// The function returns the global slot number of a chain or block.
    /// The input X is either a chain or block and the output is the global slot number.
    fn global_slot(&self) -> Option<&GlobalSlot>;
    /// The function computes the epoch slot number of a block.
    /// The output is the epoch slot number in [0, slots_per_epoch].
    fn epoch_slot(&self) -> Option<u32>;
    /// The function the length of a chain. The output is the length of the chain in blocks.
    fn length(&self) -> usize;
    /// This function returns the hex digest of the hash of the last VRF output
    ///  of a given chain. The input is a chain C and the output is the hash digest.
    fn last_vrf_hash(&self) -> Option<String>;
    /// This function returns hash of the top block's protocol state for a given chain.
    /// The input is a chain C and the output is the hash.
    fn state_hash(&self) -> Option<StateHash>;
    fn genesis_block(&self) -> Option<&ProtocolState>;
}

impl Chain<ProtocolState> for ProtocolStateChain {
    fn push(&mut self, new: ProtocolState) -> Result<(), ConsensusError> {
        match self.0.len() {
            0 => (),
            n => {
                if new.get_height().0 != self.0[n - 1].get_height().0 + 1 {
                    return Err(ConsensusError::InvalidHeight);
                }
            }
        }

        self.0.push(new);
        Ok(())
    }

    fn top(&self) -> Option<&ProtocolState> {
        self.0.last()
    }

    fn genesis_block(&self) -> Option<&ProtocolState> {
        self.0.first()
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

    fn last_vrf_hash(&self) -> Option<String> {
        self.top().map(|s| {
            s.body
                .consensus_state
                .last_vrf_output
                .hash()
                .as_ref()
                .encode_hex::<String>()
        })
    }

    // FIXME: this currently returns a blake2b hash. It should
    // return a poseidon hash according to: https://github.com/MinaProtocol/mina/blob/32f529d8d8d712a44ee75be66061ce08cbdc8924/docs/specs/consensus/README.md#517-hashstate
    fn state_hash(&self) -> Option<StateHash> {
        self.top().map(|s| s.hash())
    }
}

pub trait Consensus {
    type Chain;
    fn select_secure_chain<'a>(
        &'a self,
        candidates: &'a [Self::Chain],
        constants: &ConsensusConstants,
    ) -> Result<&'a ProtocolStateChain, ConsensusError>;

    fn select_longer_chain<'a>(
        &'a self,
        candidate: &'a ProtocolStateChain,
    ) -> Result<&'a ProtocolStateChain, ConsensusError>;
}

// TODO: replace from checkpoint.rs
fn is_short_range(_cs: &ProtocolStateChain) -> bool {
    false
}

impl Consensus for ProtocolStateChain {
    type Chain = ProtocolStateChain;
    fn select_secure_chain<'a>(
        &'a self,
        candidates: &'a [Self::Chain],
        constants: &ConsensusConstants,
    ) -> Result<&'a ProtocolStateChain, ConsensusError> {
        let tip = candidates.iter().fold(Ok(self), |tip, c| {
            if is_short_range(c) {
                // short-range fork, select longer chain
                self.select_longer_chain(c)
            } else {
                // long-range fork, compare relative minimum window densities
                let tip_state = self
                    .consensus_state()
                    .ok_or(ConsensusError::ConsensusStateNotFound)?;
                let candidate_state = c
                    .consensus_state()
                    .ok_or(ConsensusError::ConsensusStateNotFound)?;
                let tip_density =
                    relative_min_window_density(tip_state, candidate_state, constants)?;
                let candidate_density =
                    relative_min_window_density(candidate_state, tip_state, constants)?;
                match candidate_density.cmp(&tip_density) {
                    std::cmp::Ordering::Greater => Ok(c),
                    std::cmp::Ordering::Equal => self.select_longer_chain(c),
                    _ => tip, // no change
                }
            }
        });

        tip
    }

    fn select_longer_chain<'a>(
        &'a self,
        candidate: &'a ProtocolStateChain,
    ) -> Result<&'a ProtocolStateChain, ConsensusError> {
        let top_state = self
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        let candidate_state = candidate
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        if top_state.blockchain_length < candidate_state.blockchain_length {
            return Ok(candidate);
        } else if top_state.blockchain_length == candidate_state.blockchain_length {
            // tiebreak logic
            match candidate.last_vrf_hash().cmp(&self.last_vrf_hash()) {
                std::cmp::Ordering::Greater => return Ok(candidate),
                std::cmp::Ordering::Equal => {
                    if candidate.state_hash() > self.state_hash() {
                        return Ok(candidate);
                    }
                }
                _ => {}
            }
        }

        Ok(self)
    }
}
