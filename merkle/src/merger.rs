// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// Trait that merges the hashes of a pair of nodes
/// and calculates the hash of their parent
pub trait MerkleMerger {
    /// Type that represents the hash value
    type Hash;
    /// Merges hashes and metadata from a pair of nodes
    fn merge(
        left: &Option<(Self::Hash, MerkleTreeNodeMetadata)>,
        right: &Option<(Self::Hash, MerkleTreeNodeMetadata)>,
    ) -> Option<Self::Hash>;
}
