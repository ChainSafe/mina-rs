// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::prefixes::*;
use lockfree_object_pool::SpinLockObjectPool;
use mina_hasher::{create_legacy, Fp, Hashable, Hasher, PoseidonHasherLegacy, ROInput};
use once_cell::sync::OnceCell;

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

impl MerkleMerger for MinaPoseidonMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; MINA_POSEIDON_MERKLE_DEGREE],
        metadata: MerkleTreeNodeMetadata<MINA_POSEIDON_MERKLE_DEGREE>,
    ) -> Option<Self::Hash> {
        static HASHER_POOL: OnceCell<
            SpinLockObjectPool<PoseidonHasherLegacy<MinaPoseidonMerkleTreeNonLeafNode>>,
        > = OnceCell::new();
        // Not calling reset here because `hasher.init` is called after `pull`, which implicitly calls sponge.reset()
        let pool = HASHER_POOL.get_or_init(|| SpinLockObjectPool::new(|| create_legacy(0), |_| ()));
        let height = metadata.height();
        let hashable = MinaPoseidonMerkleTreeNonLeafNode(hashes, metadata);
        let mut hasher = pool.pull();
        hasher.init(height);
        Some(hasher.hash(&hashable))
    }
}

#[derive(Clone)]
struct MinaPoseidonMerkleTreeNonLeafNode(
    [Option<Fp>; MINA_POSEIDON_MERKLE_DEGREE],
    MerkleTreeNodeMetadata<MINA_POSEIDON_MERKLE_DEGREE>,
);

impl Hashable for MinaPoseidonMerkleTreeNonLeafNode {
    type D = u32;

    fn to_roinput(&self) -> mina_hasher::ROInput {
        let mut roi = ROInput::new();
        for hash in self.0.into_iter().flatten() {
            roi.append_field(hash);
        }
        roi
    }

    fn domain_string(_: Option<&Self>, height: Self::D) -> Option<String> {
        // use height - 1 here because in mina leaf nodes are not counted
        if height > 0 {
            Some(make_prefix_merkle_tree(height - 1))
        } else {
            None
        }
    }
}
