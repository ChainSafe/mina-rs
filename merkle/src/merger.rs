// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// Trait that merges the hashes of child nodes
/// and calculates the hash of their parent
pub trait MerkleMerger<const DEGREE: usize> {
    /// Type that represents the hash value
    type Hash;
    /// Merges hashes and metadata from a child nodes
    fn merge(
        items: [&Option<(Self::Hash, MerkleTreeNodeMetadata<DEGREE>)>; DEGREE],
    ) -> Option<Self::Hash>;
}
