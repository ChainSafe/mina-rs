// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod traits;
pub use crate::address as Addr;
use traits::S;
pub mod prefix {
    const GENERIC: u8 = 0xff;
    const ACCOUNT: u8 = 0xfe;
    pub fn hash(ledger_depth: u8, depth: u8) -> u8 {
        ledger_depth - depth
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum T {
    Generic,
    Account,
    Hash,
}

impl S for T {
    fn is_generic(&self) -> bool {
        match *self {
            Self::Generic => true,
            _ => false,
        }
    }

    fn is_account(&self) -> bool {
        match *self {
            Self::Account => true,
            _ => false,
        }
    }

    fn is_hash(&self) -> bool {
        match *self {
            Self::Hash => true,
            _ => false,
        }
    }

    fn height(ledger_depth: usize, other: traits::T) -> usize {
        todo!()
    }

    fn root_hash(&self) -> traits::T {
        todo!()
    }

    fn last_direction(addr: &Addr::stable::v1::MerkleAddress) -> direction::Direction {
        todo!()
    }

    fn to_path_exn(&self) -> Addr::stable::v1::MerkleAddress {
        todo!()
    }

    fn parent(other: traits::T) -> traits::T {
        todo!()
    }

    fn next(other: traits::T) -> traits::T {
        todo!()
    }

    fn prev(other: traits::T) -> traits::T {
        todo!()
    }

    fn sibling(other: traits::T) -> traits::T {
        todo!()
    }
}

pub use T as Location;
