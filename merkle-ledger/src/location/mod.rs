// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod traits;
pub use crate::address as Addr;
use traits::S;
pub mod prefix {
    // const GENERIC: u8 = 0xff;
    // const ACCOUNT: u8 = 0xfe;
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
        matches!(*self, Self::Generic)
    }

    fn is_account(&self) -> bool {
        matches!(*self, Self::Account)
    }

    fn is_hash(&self) -> bool {
        matches!(*self, Self::Hash)
    }

    fn height(_ledger_depth: usize, _other: traits::T) -> usize {
        todo!()
    }

    fn root_hash(&self) -> traits::T {
        todo!()
    }

    fn last_direction(_addr: &Addr::stable::v1::MerkleAddress) -> direction::Direction {
        todo!()
    }

    fn to_path_exn(&self) -> Addr::stable::v1::MerkleAddress {
        todo!()
    }

    fn parent(_other: traits::T) -> traits::T {
        todo!()
    }

    fn next(_other: traits::T) -> traits::T {
        todo!()
    }

    fn prev(_other: traits::T) -> traits::T {
        todo!()
    }

    fn sibling(_other: traits::T) -> traits::T {
        todo!()
    }
}

pub use T as Location;
