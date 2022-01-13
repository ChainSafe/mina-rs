// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait MerkleHasher {
    type Item;
    type Hash;
    fn hash(item: &Self::Item) -> Self::Hash;
}
