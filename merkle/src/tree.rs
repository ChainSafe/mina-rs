// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// Trait for implementing general binary merkle tree
pub trait MerkleTree {
    /// Type of the leaf data
    type Item;
    /// Type of the hash values
    type Hash;

    /// Depth of the tree
    fn depth(&self) -> u32;
    /// Number of leafs
    fn count(&self) -> usize;
    /// Root hash, lazy-evaluated
    fn root(&mut self) -> Option<Self::Hash>;

    /// Add a new leaf
    fn add(&mut self, item: Self::Item);
    /// Add a batch of leaves in the give order
    fn add_batch(&mut self, items: Vec<Self::Item>);
}
