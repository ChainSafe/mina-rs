// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::prefixes::*;
use mina_hasher::{create_legacy, Fp, Hashable, Hasher, ROInput};

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
/// with mina specific domain string calculated from node height
pub struct MinaPoseidonMerkleMerger {}

impl<const DEGREE: usize> MerkleMerger<DEGREE> for MinaPoseidonMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; DEGREE],
        metadata: MerkleTreeNodeMetadata<DEGREE>,
    ) -> Option<Self::Hash> {
        // FIXME: Get hasher from object pool
        // when https://github.com/o1-labs/proof-systems/pull/462/files is merged
        // FIXME: Avoid creating hasher with height when https://github.com/o1-labs/proof-systems/pull/479 is merged
        let height = metadata.height();
        let hashable = MinaPoseidonMerkleTreeNonLeafNode(hashes, metadata);
        let mut hasher = create_legacy(height);
        Some(hasher.hash(&hashable))
    }
}

#[derive(Debug, Clone)]
struct MinaPoseidonMerkleTreeNonLeafNode<const DEGREE: usize>(
    [Option<Fp>; DEGREE],
    MerkleTreeNodeMetadata<DEGREE>,
);

impl<const DEGREE: usize> Hashable for MinaPoseidonMerkleTreeNonLeafNode<DEGREE> {
    type D = u32;

    fn to_roinput(&self) -> mina_hasher::ROInput {
        let mut roi = ROInput::new();
        for hash in self.0.into_iter().flatten() {
            roi.append_field(hash);
        }
        roi
    }

    fn domain_string(_: Option<&Self>, height: Self::D) -> Option<String> {
        // FIXME: Read depth from self when https://github.com/o1-labs/proof-systems/pull/479 is merged
        // use height - 1 here because in mina leaf nodes are not counted
        Some(make_prefix_merkle_tree(height - 1))
        // if let Some(this) = this {
        //     println!("domain_string Some");
        //     let meta = &this.1;
        //     Some(make_prefix_merkle_tree(meta.height()-1))
        // } else {
        //     println!("domain_string None");
        //     None
        // }
    }
}
