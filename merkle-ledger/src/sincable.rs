// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::path::Hash as PHash;
pub trait S {
    type RootHash;
    type Hash: PHash<Self::H>;
    type Account;
    type Addr;
    type T;
    type Path;
    type H: std::hash::Hash + Eq;

    fn depth(this: &Self::T) -> usize;
    fn num_accounts(this: &Self::T) -> usize;
    fn merkle_path_at_addr_exn(this: &Self::T, addr: &Self::Addr) -> Self::Path;
    fn get_inner_hash_at_addr_exn(this: &Self::T, addr: &Self::Addr) -> Self::Hash;
    fn set_inner_hash_at_addr_exn(this: &Self::T, addr: &Self::Addr, hash: Self::Hash);
    fn set_all_accounts_rooted_at_exn(
        this: &Self::T,
        addr: &Self::Addr,
        accounts: &[Self::Account],
    );
    fn set_batch_accounts(this: &Self::T, all: &[(Self::Addr, Self::Account)]);
    fn get_all_accounts_rooted_at_exn(
        this: &Self::T,
        addr: &Self::Addr,
    ) -> Vec<(Self::Addr, Self::Account)>;
    fn merkle_root(this: &Self::T) -> Self::RootHash;
    fn make_space_for(this: &Self::T, space: usize);
}
