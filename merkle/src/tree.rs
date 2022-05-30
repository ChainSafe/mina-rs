// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::*;

/// Trait for implementing binary merkle tree
pub trait MerkleTree {
    /// Type of the leaf data
    type Item;
    /// Type of the hash values
    type Hash;

    /// Height of the tree, leaf nodes that store data are counted
    fn height(&self) -> u32;
    /// Number of leafs
    fn count(&self) -> usize;
    /// Root hash, lazy-evaluated
    fn root(&mut self) -> Option<Self::Hash>;
    /// Adds a new leaf
    fn add(&mut self, item: Self::Item) {
        self.add_batch(vec![item])
    }
    /// Adds a batch of leaves in the give order
    fn add_batch(&mut self, items: impl IntoIterator<Item = Self::Item>);
}

/// Trait for implementing sparse binary merkle tree.
/// It is essentially a collection of [MerkleProof]
pub trait SparseMerkleTree {
    /// Type of the leaf data
    type Item;
    /// Type of the hash values
    type Hash: PartialEq + Clone;
    /// Type of the merkle hasher
    type Hasher: MerkleHasher<Item = Self::Item, Hash = Self::Hash>;
    /// Type of the merkle merger
    type Merger: MerkleMerger<Hash = Self::Hash>;

    /// Adds a single [MerkleProof]
    fn add(
        &mut self,
        proof: DefaultMerkleProof<Self::Item, Self::Hash, Self::Hasher, Self::Merger>,
    ) {
        self.add_batch(vec![proof])
    }

    /// Adds a collection of [MerkleProof]
    fn add_batch(
        &mut self,
        proofs: impl IntoIterator<
            Item = DefaultMerkleProof<Self::Item, Self::Hash, Self::Hasher, Self::Merger>,
        >,
    );
}
