// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Implements common APIs for the blockchain in the context of consensus.

use crate::error::ConsensusError;
use hex::ToHex;
use mina_crypto::hash::{Hashable, StateHash};
use mina_rs_base::consensus_state::ConsensusState;
use mina_rs_base::global_slot::GlobalSlot;
use mina_rs_base::protocol_state::{Header, ProtocolState};
use mina_rs_base::types::{BlockTime, Length};

// TODO: derive from protocol constants
pub struct ConsensusConstants {
    /// Point of finality (number of confirmations)
    pub k: Length,
    /// Number of slots per epoch
    pub slots_per_epoch: Length,
    /// No of slots in a sub-window = 7
    pub slots_per_sub_window: Length,
    /// Maximum permissable delay of packets (in slots after the current)
    pub delta: Length,
    /// Timestamp of genesis block in unixtime
    pub genesis_state_timestamp: BlockTime,
    /// Sub windows within a window
    pub sub_windows_per_window: Length,
    /// Number of slots before minimum density is used in chain selection
    pub grace_period_end: Length,
}

impl ConsensusConstants {
    pub fn mainnet() -> Self {
        Self {
            k: Length(290),
            slots_per_epoch: Length(7140),
            slots_per_sub_window: Length(7),
            delta: Length(0),
            genesis_state_timestamp: BlockTime(1615939200000),
            sub_windows_per_window: Length(11),
            grace_period_end: Length(1440),
        }
    }

    pub fn devnet() -> Self {
        todo!()
    }
}

#[derive(Debug, Default)]
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
    /// Top level API to select between chains during a fork.
    fn select_secure_chain<'a>(
        &'a self,
        candidates: &'a [Self::Chain],
    ) -> Result<&'a ProtocolStateChain, ConsensusError>;

    /// Selects the longer chain when there's a short range fork.
    fn select_longer_chain<'a>(
        &'a self,
        candidate: &'a ProtocolStateChain,
    ) -> Result<&'a ProtocolStateChain, ConsensusError>;

    /// Checks whether the fork is short range wrt to candidate chain
    fn is_short_range(&self, candidate: &ProtocolStateChain) -> Result<bool, ConsensusError>;

    /// Calculates the relate minimum window density wrt to candidate chain.
    fn relative_min_window_density(
        &self,
        candidate: &ProtocolStateChain,
    ) -> Result<u32, ConsensusError>;

    /// Constants used for consensus
    fn config(&self) -> ConsensusConstants;
}

impl Consensus for ProtocolStateChain {
    type Chain = ProtocolStateChain;
    fn select_secure_chain<'a>(
        &'a self,
        candidates: &'a [Self::Chain],
    ) -> Result<&'a ProtocolStateChain, ConsensusError> {
        let tip = candidates.iter().fold(Ok(self), |tip, candidate| {
            if self.is_short_range(candidate)? {
                // short-range fork, select longer chain
                self.select_longer_chain(candidate)
            } else {
                let tip_density = self.relative_min_window_density(candidate)?;
                let candidate_density = candidate.relative_min_window_density(self)?;
                match candidate_density.cmp(&tip_density) {
                    std::cmp::Ordering::Greater => Ok(candidate),
                    std::cmp::Ordering::Equal => self.select_longer_chain(candidate),
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

    fn is_short_range(&self, candidate: &ProtocolStateChain) -> Result<bool, ConsensusError> {
        let a = &self
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        let b = &candidate
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        let a_prev_lock_checkpoint = &a.staking_epoch_data.lock_checkpoint;
        let b_prev_lock_checkpoint = &b.staking_epoch_data.lock_checkpoint;

        let check = |s1: &ConsensusState, s2: &ConsensusState, s2_epoch_slot: Option<u32>| {
            if s1.epoch_count.0 == s2.epoch_count.0 + 1
                && s2_epoch_slot >= Some(self.config().slots_per_epoch.0 * 2 / 3)
            {
                // S1 is one epoch ahead of S2 and S2 is not in the seed update range
                s1.staking_epoch_data.lock_checkpoint == s2.next_epoch_data.lock_checkpoint
            } else {
                false
            }
        };

        if a.epoch_count == b.epoch_count {
            // Simple case: blocks have same previous epoch, so compare previous epochs' lock_checkpoints
            Ok(a_prev_lock_checkpoint == b_prev_lock_checkpoint)
        } else {
            // Check for previous epoch case using both orientations
            Ok(check(a, b, candidate.epoch_slot()) || check(b, a, self.epoch_slot()))
        }
    }

    fn config(&self) -> ConsensusConstants {
        ConsensusConstants::mainnet()
    }

    /// Computes the relative minimum window density of the given chains.
    /// The minimum density value is used in the case of a long range fork
    /// and the chain with the higher relative minimum window density is chosen as the canonical chain.
    /// The need for relative density is explained here:
    /// <https://github.com/MinaProtocol/mina/blob/02dfc3ff0160ba3c1bbc732baa07502fe4312b04/docs/specs/consensus/README.md#5412-relative-minimum-window-density>
    fn relative_min_window_density(
        &self,
        chain_b: &ProtocolStateChain,
    ) -> Result<u32, ConsensusError> {
        let tip_state = self
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        let chain_b = chain_b
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;

        // helpers for readability.
        let min = |a: u32, b: u32| a.min(b);
        let max = |a: u32, b: u32| a.max(b);

        let max_slot = max(
            tip_state.curr_global_slot.slot_number.0,
            chain_b.curr_global_slot.slot_number.0,
        );

        // grace-period rule
        if max_slot < self.config().grace_period_end.0 {
            return Ok(tip_state.min_window_density.0);
        }

        let projected_window = {
            // compute shift count
            let mut shift_count = min(
                max(
                    max_slot - tip_state.curr_global_slot.slot_number.0.saturating_sub(1),
                    0,
                ),
                self.config().sub_windows_per_window.0,
            );
            // initialize projected window based off of chain_a
            let mut projected_window = tip_state.sub_window_densities.clone();

            // relative sub window
            let mut rel_sub_window = tip_state.curr_global_slot.slot_number.0
                / self.config().sub_windows_per_window.0
                % self.config().sub_windows_per_window.0;

            // ring shift
            while shift_count > 0 {
                rel_sub_window = (rel_sub_window + 1) % self.config().sub_windows_per_window.0;
                projected_window[rel_sub_window as usize] = Length(0);
                shift_count -= 1;
            }

            projected_window
        };

        // compute projected window density
        let projected_window_density = projected_window.iter().map(|s| s.0).sum();

        // compute minimum window density
        Ok(min(
            tip_state.min_window_density.0,
            projected_window_density,
        ))
    }
}
