// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// Trait that merges the hashes of child nodes
/// and calculates the hash of their parent
pub trait MerkleMerger {
    /// Type that represents the hash value
    type Hash;
    /// Merges hashes of child nodes,
    /// with metadata of the target node
    fn merge(
        hashes: [Option<Self::Hash>; 2],
        metadata: MerkleTreeNodeMetadata,
    ) -> Option<Self::Hash>;
}
