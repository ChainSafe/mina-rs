// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait Hash {
    type T: Eq + Into<&'static str> + Clone;
    type Account;
    fn to_string(other: &Self::T) -> String;
    fn merge(height: usize, other: &Self, another: &Self) -> Self;
    fn hash_account(account: &Self::Account) -> Self::T;
    fn empty_account() -> Self::T;
    // include Hashable.S_binable
}
