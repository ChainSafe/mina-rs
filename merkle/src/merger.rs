// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::prefixes::*;
use mina_curves::pasta::Fp;
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};

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

/// Merger for mina binary merkle tree that uses poseidon hash
/// with mina specific domain string calculated from node depth
pub struct MinaPoseidonMerkleMerger {}

impl MerkleMerger for MinaPoseidonMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; 2],
        metadata: MerkleTreeNodeMetadata<2>,
    ) -> Option<Self::Hash> {
        // FIXME: Get hasher from object pool
        // when https://github.com/o1-labs/proof-systems/pull/462/files is merged
        let mut hasher = create_legacy(());
        Some(hasher.hash(&MinaPoseidonMerkleTreeNonLeafNode(hashes, metadata)))
    }
}

#[derive(Debug, Clone)]
struct MinaPoseidonMerkleTreeNonLeafNode<const DEGREE: usize>(
    [Option<Fp>; DEGREE],
    MerkleTreeNodeMetadata<2>,
);

impl<const DEGREE: usize> Hashable for MinaPoseidonMerkleTreeNonLeafNode<DEGREE> {
    type D = ();

    fn to_roinput(&self) -> mina_hasher::ROInput {
        let mut roi = ROInput::new();
        for hash in self.0.into_iter().flatten() {
            roi.append_field(hash)
        }
        roi
    }

    fn domain_string(this: Option<&Self>, _: Self::D) -> Option<String> {
        if let Some(this) = this {
            let meta = &this.1;
            Some(make_prefix_merkle_tree(meta.depth()))
        } else {
            None
        }
    }
}
