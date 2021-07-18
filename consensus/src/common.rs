// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use blake2_rfc::blake2b::blake2b;
use mina_crypto::hash::BaseHash;
use mina_rs_base::consensus_state::ConsensusState;
use mina_rs_base::global_slot::GlobalSlot;
use mina_rs_base::protocol_state::{Header, ProtocolState};
use serde_bin_prot::to_writer;
use std::convert::TryInto;

pub struct ProtocolStateChain(Vec<ProtocolState>);

pub trait Chain<T>
where
    T: Header,
{
    fn push(&mut self, new: T) -> Result<(), &'static str>;
    fn top(&self) -> Option<&T>;
    fn consensus_state(&self) -> Option<&ConsensusState>;
    fn global_slot(&self) -> Option<&GlobalSlot>;
    fn epoch_slot(&self) -> Option<u32>;
    fn length(&self) -> u64;
    fn last_vrf(&self) -> String;
    fn state_hash(&self) -> Option<BaseHash>;
}

impl Chain<ProtocolState> for ProtocolStateChain {
    fn push(&mut self, new: ProtocolState) -> Result<(), &'static str> {
        match self.0.len() {
            0 => Ok(self.0.push(new)),
            n => {
                if new.get_height().0 != self.0[n - 1].get_height().0 + 1 {
                    return Err("header must have height 1 greater than top");
                }

                Ok(self.0.push(new))
            }
        }
    }

    fn top(&self) -> Option<&ProtocolState> {
        self.0.last()
    }

    fn consensus_state(&self) -> Option<&ConsensusState> {
        match self.top() {
            Some(s) => Some(&s.body.consensus_state),
            None => None,
        }
    }

    fn global_slot(&self) -> Option<&GlobalSlot> {
        match self.top() {
            Some(s) => Some(&s.body.consensus_state.curr_global_slot),
            None => None,
        }
    }

    fn epoch_slot(&self) -> Option<u32> {
        self.global_slot()
            .map(|s| (s.slot_number.0 % s.slots_per_epoch.0).try_into().unwrap())
    }

    fn length(&self) -> u64 {
        self.0.len().try_into().unwrap()
    }

    fn last_vrf(&self) -> String {
        let s = match self.top() {
            Some(s) => s,
            None => return "0x".to_string(),
        };

        let hash = blake2b(
            32,
            &[],
            &s.body.consensus_state.last_vrf_output.0.as_bytes(),
        );
        BaseHash::from(hash.as_bytes()).to_hex()
    }

    fn state_hash(&self) -> Option<BaseHash> {
        let s = match self.top() {
            Some(s) => s,
            None => return None,
        };

        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &s).unwrap();
        let hash = blake2b(32, &[], &output);
        Some(BaseHash::from(hash.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_rs_base::numbers::{GlobalSlot as GlobalSlotNumber, Length};

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
        assert_eq!(
            c.push(b1),
            Err("header must have height 1 greater than top")
        );
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
        assert_eq!(String::from("0x"), c.last_vrf());

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();

        let hash = blake2b(32, &[], String::new().as_bytes());
        let expected = BaseHash::from(hash.as_bytes()).to_hex();
        assert_eq!(expected, c.last_vrf());
    }
}
