// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub trait MerkleHasher {
    type Item;
    type Hash;
    fn hash(item: &Self::Item, metadata: MerkleTreeNodeMetadata) -> Self::Hash;
}
