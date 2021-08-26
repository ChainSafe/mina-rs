// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub use crate::location::Addr;

pub trait Prefix {
    fn generic() -> u8;
    fn account() -> u8;
    fn hash(ledger_depth: usize, other: usize) -> u8;
}

pub enum T {
    // Generic(BigString),
    Account(Addr::stable::v1::MerkleAddress),
    Hash(Addr::stable::v1::MerkleAddress),
}

pub trait S {
    fn is_generic(&self) -> bool;
    fn is_account(&self) -> bool;
    fn is_hash(&self) -> bool;
    fn height(ledger_depth: usize, other: T) -> usize;
    fn root_hash(&self) -> T;
    fn last_direction(addr: &Addr::stable::v1::MerkleAddress) -> direction::Direction;
    // fn build_generic(other: BigString) -> T;
    // fn parse(ledger_depth: usize, other: BigString) -> anyhow::Result<T>;
    // fn prefix_bigstring(prefix: u8, this: BigString) -> BigString;
    fn to_path_exn(&self) -> Addr::stable::v1::MerkleAddress;
    // fn serialize(ledger_depth: usize, other: T) -> BigString;
    fn parent(other: T) -> T;
    fn next(other: T) -> T;
    fn prev(other: T) -> T;
    fn sibling(other: T) -> T;
    // fn order_siblings(other: T) -> T;
}
