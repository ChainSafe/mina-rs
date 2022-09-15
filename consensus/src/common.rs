// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Implements common APIs for the blockchain in the context of consensus.
//!

use crate::error::ConsensusError;
use mina_rs_base::consensus_state::ConsensusState;
use mina_rs_base::global_slot::GlobalSlot;
use mina_rs_base::protocol_state::ProtocolStateHeader;
use mina_rs_base::types::{BlockTime, Length};
use proof_systems::mina_hasher::Fp;

// TODO: derive from protocol constants
/// Constants used for the conensus
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
    /// Pre-defined constant values for mainnet
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

    /// Pre-defined constant values for devnet
    pub fn devnet() -> Self {
        todo!()
    }
}

/// A chain of ProtocolState
#[derive(Debug, Default, Eq, PartialEq, Clone)]
// TODO: replace vec element with ExternalTransition
pub struct ProtocolStateChain<T>(pub Vec<T>)
where
    T: ProtocolStateHeader;

impl<T> ProtocolStateChain<T>
where
    T: ProtocolStateHeader,
{
    /// Pushes an item into the chain
    pub fn push(&mut self, new: T) -> Result<(), ConsensusError> {
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

    /// This function returns the last block of a given chain.
    /// The input is a chain C and the output is last block of C
    /// (i.e. the block with greatest height).
    pub fn top(&self) -> Option<&T> {
        self.0.last()
    }

    /// he function returns the consensus state of a block or chain.
    /// The input is a block or chain X and the output is the consensus state.
    pub fn consensus_state(&self) -> Option<&ConsensusState> {
        self.top().map(|s| s.consensus_state())
    }

    /// Gets ProtocolState of the genesis block
    pub fn genesis_block(&self) -> Option<&T> {
        self.0.first()
    }

    /// The function returns the global slot number of a chain or block.
    /// The input X is either a chain or block and the output is the global slot number.
    pub fn global_slot(&self) -> Option<&GlobalSlot> {
        self.top().map(|s| &s.consensus_state().curr_global_slot)
    }

    /// The function computes the epoch slot number of a block.
    /// The output is the epoch slot number in [0, slots_per_epoch].
    pub fn epoch_slot(&self) -> Option<u32> {
        self.global_slot()
            .map(|s| (s.slot_number.0 % s.slots_per_epoch.0))
    }

    /// The function the length of a chain. The output is the length of the chain in blocks.
    pub fn length(&self) -> usize {
        self.consensus_state()
            .map(|s| s.blockchain_length.0 as usize)
            .unwrap_or(0)
    }

    /// This function returns the hex digest of the hash of the last VRF output
    ///  of a given chain. The input is a chain C and the output is the hash digest.
    pub fn last_vrf_hash_digest(&self) -> Result<String, ConsensusError> {
        let hash = self
            .consensus_state()
            .ok_or(ConsensusError::TopBlockNotFound)?
            .last_vrf_output
            .digest();
        Ok(hex::encode(hash))
    }

    /// This function returns hash of the top block's protocol state for a given chain.
    /// The input is a chain C and the output is the hash.
    pub fn state_hash(&self) -> Option<Fp> {
        self.top().map(|s| s.state_hash_fp())
    }
}

/// A trait that defines operations for chain selection
pub trait ChainSelection {
    /// Top level API to select between chains during a fork.
    fn select_secure_chain(&mut self, candidates: Vec<Self>) -> Result<(), ConsensusError>
    where
        Self: Sized;

    /// Selects the longer chain when there's a short range fork.
    fn select_longer_chain(&mut self, candidate: Self) -> Result<(), ConsensusError>
    where
        Self: Sized;

    /// Checks whether the fork is short range wrt to candidate chain
    fn is_short_range(&self, candidate: &Self) -> Result<bool, ConsensusError>;

    /// Calculates the relate minimum window density wrt to candidate chain.
    fn relative_min_window_density(&self, candidate: &Self) -> Result<u32, ConsensusError>;

    /// Constants used for consensus
    fn config(&self) -> ConsensusConstants;
}

impl<T> ChainSelection for ProtocolStateChain<T>
where
    T: ProtocolStateHeader,
{
    fn select_secure_chain(&mut self, candidates: Vec<Self>) -> Result<(), ConsensusError> {
        for candidate in candidates {
            if self.is_short_range(&candidate)? {
                // short-range fork, select longer chain
                self.select_longer_chain(candidate)?;
            } else {
                // check against sub window density sizes > 11
                let candidate_state = candidate
                    .consensus_state()
                    .ok_or(ConsensusError::ConsensusStateNotFound)?;

                // sub window density must not be greater than initial genesis subwindow density value.
                if candidate_state
                    .sub_window_densities()
                    .iter()
                    .any(|s| *s > self.config().slots_per_sub_window.0)
                {
                    continue;
                };

                // sub window densities must not be greater than sub_windows_per_window
                let sub_windows_per_window = self.config().sub_windows_per_window.0 as usize;
                if candidate_state.sub_window_densities.len() != sub_windows_per_window {
                    continue;
                }

                let tip_density = self.relative_min_window_density(&candidate)?;
                let candidate_density = candidate.relative_min_window_density(self)?;
                match candidate_density.cmp(&tip_density) {
                    std::cmp::Ordering::Greater => *self = candidate,
                    std::cmp::Ordering::Equal => self.select_longer_chain(candidate)?,
                    _ => (), // no change
                }
            }
        }
        Ok(())
    }

    fn select_longer_chain(&mut self, candidate: Self) -> Result<(), ConsensusError> {
        let top_state = self
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        let candidate_state = candidate
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        if top_state.blockchain_length < candidate_state.blockchain_length {
            *self = candidate;
            return Ok(());
        } else if top_state.blockchain_length == candidate_state.blockchain_length {
            // tiebreak logic
            match candidate
                .last_vrf_hash_digest()?
                .cmp(&self.last_vrf_hash_digest()?)
            {
                std::cmp::Ordering::Greater => {
                    *self = candidate;
                    return Ok(());
                }
                std::cmp::Ordering::Equal => {
                    if candidate.state_hash() > self.state_hash() {
                        *self = candidate;
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn is_short_range(&self, candidate: &Self) -> Result<bool, ConsensusError> {
        let a = self
            .consensus_state()
            .ok_or(ConsensusError::ConsensusStateNotFound)?;
        let b = candidate
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
        // FIXME: this is OK for now as ConsensusConstants is identical between mainnet and qanet
        ConsensusConstants::mainnet()
    }

    /// Computes the relative minimum window density of the given chains.
    /// The minimum density value is used in the case of a long range fork
    /// and the chain with the higher relative minimum window density is chosen as the canonical chain.
    /// The need for relative density is explained here:
    /// <https://github.com/MinaProtocol/mina/blob/02dfc3ff0160ba3c1bbc732baa07502fe4312b04/docs/specs/consensus/README.md#5412-relative-minimum-window-density>
    fn relative_min_window_density(&self, chain_b: &Self) -> Result<u32, ConsensusError> {
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
                match projected_window.get_mut(rel_sub_window as usize) {
                    Some(density) => *density = Length(0),
                    None => return Err(ConsensusError::CandidatesMissingSubWindowDensities),
                };
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
