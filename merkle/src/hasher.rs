// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// Trait for implementing merkle tree hasher
/// degree defaults to 2
pub trait MerkleHasher<const DEGREE: usize = DEFAULT_DEGREE> {
    /// Type that [MerkleHasher] calculates hash from
    type Item;
    /// Type that represents the hash value
    type Hash;
    /// Calculates hash from an item and its associated metadata
    fn hash(item: &Self::Item, metadata: MerkleTreeNodeMetadata<DEGREE>) -> Self::Hash;
}
