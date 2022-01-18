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
    // The function computes the epoch slot number of a block.
    // The output is the epoch slot number in [0, slots_per_epoch].
    fn epoch_slot(&self) -> Option<u32>;
    // The function the length of a chain. The output is the length of the chain in blocks.
    fn length(&self) -> usize;
    // This function returns the hex digest of the hash of the last VRF output
    //  of a given chain. The input is a chain C and the output is the hash digest.
    fn last_vrf_hash(&self) -> Option<String>;
    // This function returns hash of the top block's protocol state for a given chain.
    // The input is a chain C and the output is the hash.
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

trait Consensus {
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
        let mut tip = self;

        for c in candidates {
            if is_short_range(c) {
                // short-range fork, select longer chain
                tip = self.select_longer_chain(c)?;
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
                    std::cmp::Ordering::Greater => tip = c,
                    std::cmp::Ordering::Equal => tip = self.select_longer_chain(c)?,
                    _ => {}
                }
            }
        }

        Ok(tip)
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

#[cfg(test)]
mod tests {
    use super::*;
    use mina_rs_base::{numbers::Length, types::GlobalSlotNumber};

    #[test]
    fn test_protocol_state_chain_push() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(c.length(), 0);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();
        assert_eq!(c.length(), 1);

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        c.push(b1).unwrap();
        assert_eq!(c.length(), 2);

        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(2);
        c.push(b2).unwrap();
        assert_eq!(c.length(), 3);

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        assert_eq!(c.push(b1).unwrap_err(), ConsensusError::InvalidHeight,);
    }

    #[test]
    fn test_protocol_state_chain_top() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(c.length(), 0);
        assert_eq!(c.top(), None);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();
        assert_eq!(c.length(), 1);
        let expected: ProtocolState = Default::default();
        assert_eq!(c.top(), Some(&expected));

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        c.push(b1).unwrap();
        let mut expected: ProtocolState = Default::default();
        expected.body.consensus_state.blockchain_length = Length(1);
        assert_eq!(c.top(), Some(&expected));
    }

    #[test]
    fn test_protocol_state_chain_epoch_slot() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        b0.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(1000),
        };
        c.push(b0).unwrap();
        let epoch_slot = c.epoch_slot();
        assert_eq!(epoch_slot, Some(0));

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1),
            slots_per_epoch: Length(1000),
        };
        c.push(b1).unwrap();
        let epoch_slot = c.epoch_slot();
        assert_eq!(epoch_slot, Some(1));

        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(2);
        b2.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1002),
            slots_per_epoch: Length(1000),
        };
        c.push(b2).unwrap();
        let epoch_slot = c.epoch_slot();
        assert_eq!(epoch_slot, Some(2));
    }

    #[test]
    fn test_protocol_state_chain_state_hash() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();

        let hash = c.state_hash();
        hash.unwrap();
    }

    #[test]
    fn test_protocol_state_chain_last_vrf() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(None, c.last_vrf_hash());

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0.clone()).unwrap();

        let expected = Some(b0.body.consensus_state.last_vrf_output.hash().encode_hex());
        assert_eq!(expected, c.last_vrf_hash());
    }

    #[test]
    fn selects_longer_chain() {
        let constants = ConsensusConstants::from_genesis();
        let mut genesis_chain = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.min_window_density = Length(77);
        consensus_state.sub_window_densities = vec![
            Length(1),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
        ];

        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(7140),
        };

        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        genesis_chain.push(prot_state).unwrap();

        let mut chain_at_5001 = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.min_window_density = Length(43);
        let densities = vec![
            Length(5),
            Length(5),
            Length(2),
            Length(5),
            Length(3),
            Length(1),
            Length(5),
            Length(3),
            Length(7),
            Length(6),
            Length(5),
        ];

        consensus_state.sub_window_densities = densities.clone();

        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(7042),
            slots_per_epoch: Length(7140),
        };

        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_at_5001.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_at_5001);
        let select_result = genesis_chain
            .select_secure_chain(&chains, &constants)
            .unwrap();
        let a = select_result.0.get(0).unwrap();
        assert_eq!(a.body.consensus_state.min_window_density, Length(43));
        assert_eq!(a.body.consensus_state.sub_window_densities, densities);
    }
}
