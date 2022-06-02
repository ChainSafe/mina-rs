// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! This module contains traits, structs and utilities of merkle proof

use std::marker::PhantomData;

use crate::*;

/// Merkle proof trait of a single leaf node, for details refer to <https://www.webopedia.com/definitions/merkle-proof/>
pub trait MerkleProof {
    /// Hash type
    type Hash: PartialEq;

    /// Calculates the root hash
    fn root_hash(&self) -> Option<Self::Hash>;

    /// Verifies if the proof is valid
    fn verify(&self, root_hash: &Self::Hash) -> bool {
        if let Some(hash) = self.root_hash() {
            &hash == root_hash
        } else {
            false
        }
    }
}

/// Merkle proof implementation of a single leaf node, for details refer to <https://www.webopedia.com/definitions/merkle-proof/>
pub struct DefaultMerkleProof<Item, Hash, Hasher, Merger>
where
    Hash: PartialEq + Clone,
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
{
    index: usize,
    item: Item,
    peer_indices: Vec<usize>,
    peer_hashes: Vec<Option<Hash>>,
    _hasher: PhantomData<Hasher>,
    _merger: PhantomData<Merger>,
}

impl<Item, Hash, Hasher, Merger> DefaultMerkleProof<Item, Hash, Hasher, Merger>
where
    Hash: PartialEq + Clone,
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
{
    /// Creates merkle proof instance
    /// index is the node index counted from root, index of root node is 0
    pub fn new(
        index: usize,
        item: Item,
        peer_indices: Vec<usize>,
        peer_hashes: Vec<Option<Hash>>,
    ) -> Self {
        assert_eq!(peer_indices.len(), peer_hashes.len());
        Self {
            index,
            item,
            peer_indices,
            peer_hashes,
            _hasher: Default::default(),
            _merger: Default::default(),
        }
    }
}

impl<Item, Hash, Hasher, Merger> MerkleProof for DefaultMerkleProof<Item, Hash, Hasher, Merger>
where
    Hash: PartialEq + Clone + std::fmt::Debug,
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
{
    type Hash = Hash;

    fn root_hash(&self) -> Option<Hash> {
        if self.index > 0 {
            let mut index = self.index;
            let mut hash_opt = Some(Hasher::hash(
                &self.item,
                MerkleTreeNodeMetadata::new(self.index, self.peer_indices.len() as u32),
            ));
            for i in 0..self.peer_indices.len() {
                let peer_index = self.peer_indices[i];
                let peer_hash_opt = &self.peer_hashes[i];
                let hashes = if index < peer_index {
                    [hash_opt, peer_hash_opt.clone()]
                } else {
                    [peer_hash_opt.clone(), hash_opt]
                };
                let parent_index = get_parent_index(index);
                hash_opt = Merger::merge(
                    hashes,
                    MerkleTreeNodeMetadata::new(parent_index, self.peer_indices.len() as u32),
                );
                index = parent_index;
            }
            hash_opt
        } else {
            None
        }
    }
}

fn get_parent_index(index: usize) -> usize {
    debug_assert!(index > 0);
    (index - 1) / 2
}
