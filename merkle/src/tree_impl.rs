// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use std::marker::PhantomData;

/// Special complete binary merkle tree that is compatible with
/// <https://github.com/o1-labs/snarky/blob/master/src/base/merkle_tree.ml>
/// whose leaf nodes are at the same height
pub struct MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<Hash = THash>,
    THash: Clone,
{
    depth: u32,
    leafs: Vec<(THash, TItem)>,
    nodes: Vec<Option<THash>>,

    _pd_hasher: PhantomData<THasher>,
    _pd_merger: PhantomData<TMerger>,
}

impl<TItem, THash, THasher, TMerger> MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<Hash = THash>,
    THash: Clone,
{
    /// Creates a new instance of [MinaMerkleTree]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new instance of [MinaMerkleTree] with estimated capacity of leaves
    pub fn with_capacity(capacity: usize) -> Self {
        let protential_depth = calculate_depth(capacity);
        let protential_node_count = calculate_node_count(protential_depth);
        Self {
            leafs: Vec::with_capacity(capacity),
            nodes: Vec::with_capacity(protential_node_count),
            ..Default::default()
        }
    }

    fn clear_dirty_hashes(&mut self, leaf_index: usize) {
        let mut parent = leaf_index;
        while parent > 0 {
            parent = calculate_parent_index(parent);
            if self.nodes[parent].is_some() {
                self.nodes[parent] = None;
            } else {
                break;
            }
        }
    }

    fn calculate_hash_if_needed(
        &mut self,
        index: usize,
    ) -> Option<(THash, MerkleTreeNodeMetadata)> {
        if index < self.nodes.len() {
            if let Some(hash) = &self.nodes[index] {
                Some((hash.clone(), MerkleTreeNodeMetadata::new(index)))
            } else {
                let left = index * 2 + 1;
                let right = index * 2 + 2;
                let left_hash = self.calculate_hash_if_needed(left);
                let right_hash = self.calculate_hash_if_needed(right);
                let hash = TMerger::merge(&left_hash, &right_hash);
                self.nodes[index] = hash.clone();
                hash.map(|hash| (hash, MerkleTreeNodeMetadata::new(index)))
            }
        } else {
            let leaf_index = index - self.nodes.len();
            if leaf_index < self.leafs.len() {
                let (hash, _) = &self.leafs[leaf_index];
                Some((hash.clone(), MerkleTreeNodeMetadata::new(index)))
            } else {
                None
            }
        }
    }
}

impl<TItem, THash, THasher, TMerger> MerkleTree for MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<Hash = THash>,
    THash: Clone,
{
    type Item = TItem;
    type Hash = THash;

    fn depth(&self) -> u32 {
        self.depth
    }

    fn count(&self) -> usize {
        self.leafs.len()
    }

    fn root(&mut self) -> Option<Self::Hash> {
        self.calculate_hash_if_needed(0).map(|(hash, _)| hash)
    }

    fn add_batch(&mut self, items: Vec<Self::Item>) {
        let new_leaf_count = self.leafs.len() + items.len();
        let new_depth = calculate_depth(new_leaf_count);
        if new_depth != self.depth {
            let new_node_count = calculate_node_count(new_depth);
            self.depth = new_depth;
            self.nodes = vec![None; new_node_count];
        } else {
            let start = self.nodes.len() + self.leafs.len();
            for i in start..(start + items.len()) {
                self.clear_dirty_hashes(i);
            }
        }
        for leaf in items
            .into_iter()
            .enumerate()
            .map(|(i, item)| (THasher::hash(&item, MerkleTreeNodeMetadata::new(i)), item))
        {
            self.leafs.push(leaf);
        }
    }

    fn add(&mut self, item: Self::Item) {
        self.add_batch(vec![item])
    }
}

impl<TItem, THash, THasher, TMerger> Default for MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<Hash = THash>,
    THash: Clone,
{
    fn default() -> Self {
        Self {
            depth: 0,
            leafs: Vec::new(),
            nodes: Vec::new(),
            _pd_hasher: Default::default(),
            _pd_merger: Default::default(),
        }
    }
}

fn calculate_depth(size: usize) -> u32 {
    if size < 2 {
        0
    } else {
        (size as f64).log2().ceil() as u32
    }
}

fn calculate_node_count(depth: u32) -> usize {
    2_usize.pow(depth) - 1
}

fn calculate_parent_index(index: usize) -> usize {
    debug_assert!(index > 0);
    (index - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_depth_tests() {
        assert_eq!(0, calculate_depth(0));
        assert_eq!(0, calculate_depth(1));
        assert_eq!(1, calculate_depth(2));
        assert_eq!(2, calculate_depth(3));
        assert_eq!(2, calculate_depth(4));
        assert_eq!(3, calculate_depth(5));
        assert_eq!(4, calculate_depth(11));
        assert_eq!(5, calculate_depth(29));
    }
}
