// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait S {
    type Hash: std::hash::Hash + Eq;
    type Elem;
    fn elem_hash(e: Self::Elem) -> Self::Hash;
    fn implied_root(t: &[Self::Elem], leaf_hash: Self::Hash) -> Self::Hash;
    fn check_path(t: &[Self::Elem], hash: Self::Hash, other: Self::Hash) -> bool;
}
