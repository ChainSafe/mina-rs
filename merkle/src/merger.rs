// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// Trait that merges the hashes of child nodes
/// and calculates the hash of their parent
/// degree defaults to 2
pub trait MerkleMerger<const DEGREE: usize = DEFAULT_DEGREE> {
    /// Type that represents the hash value
    type Hash;
    /// Merges hashes of child nodes,
    /// with metadata of the target node
    fn merge(
        hashes: [Option<Self::Hash>; DEGREE],
        metadata: MerkleTreeNodeMetadata<DEGREE>,
    ) -> Option<Self::Hash>;
}
