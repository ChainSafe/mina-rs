// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// Trait for implementing general n-degree merkle tree
pub trait MerkleTree<const DEGREE: usize> {
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

    /// Adds a new leaf
    fn add(&mut self, item: Self::Item);
    /// Adds a batch of leaves in the give order
    fn add_batch(&mut self, items: impl IntoIterator<Item = Self::Item>);
}
