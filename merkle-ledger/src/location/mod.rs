// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod traits;
pub use crate::address as Addr;

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

impl T {
    pub fn is_generic(&self) -> bool {
        match *self {
            Self::Generic => true,
            _ => false,
        }
    }

    pub fn is_account(&self) -> bool {
        match *self {
            Self::Account => true,
            _ => false,
        }
    }

    pub fn is_hash(&self) -> bool {
        match *self {
            Self::Hash => true,
            _ => false,
        }
    }
}

// pub fn height
// pub fn root_hash
// pub fn last_direction
// pub fn build_generic
// pub fn parse
// pub fn prefix_bigstring
// pub fn to_path_exn
// pub fn serialize
// pub fn parent
// pub fn next
// pub fn prev
// pub fn sibling
// pub fn order_siblings

pub use T as Location;
