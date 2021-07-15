use blake2::{Blake2b, Digest};
use mina_crypto::hash::BaseHash;
use mina_types::protocol_state::{ConsensusState, GlobalSlot, Header, ProtocolState};
use serde_bin_prot::to_writer;
use std::convert::TryInto;

pub type ProtocolStateChain = Vec<ProtocolState>;

pub trait Chain<T>
where
    T: Header,
{
    fn push(&self, new: T) -> Result<(), &'static str>;
    fn top(&self) -> Option<&T>;
    fn consensus_state(&self) -> Option<&ConsensusState>;
    fn global_slot(&self) -> Option<&GlobalSlot>;
    fn epoch_slot(&self) -> Option<u32>;
    fn length(&self) -> u64;
    fn last_vrf(&self) -> String;
    fn state_hash(&self) -> Option<BaseHash>;
}

impl Chain<ProtocolState> for ProtocolStateChain {
    fn push(&self, new: ProtocolState) -> Result<(), &'static str> {
        match self.len() {
            0 => self.push(new),
            n => {
                if new.get_height() < self[n - 1].get_height() {
                    return Err("cannot push header with lower height than top");
                }

                self.push(new)
            }
        }
    }

    fn top(&self) -> Option<&ProtocolState> {
        self.last()
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
        self.global_slot().map(|s| (s.0 % s.1).try_into().unwrap())
    }

    fn length(&self) -> u64 {
        self.len().try_into().unwrap()
    }

    fn last_vrf(&self) -> String {
        String::new() // TODO
    }

    fn state_hash(&self) -> Option<BaseHash> {
        let s = match self.top() {
            Some(s) => s,
            None => return None,
        };

        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &s).unwrap();

        let mut hasher = Blake2b::new();
        hasher.update(output);

        // TODO: is there a prettier way to do this?
        let hash = &hasher.finalize()[..];
        let mut x: Vec<u8> = vec![0; 32];
        x[..].clone_from_slice(hash);
        Some(BaseHash::from(x.into_boxed_slice()))
    }
}
