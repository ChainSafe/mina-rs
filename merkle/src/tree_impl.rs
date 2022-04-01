// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use std::marker::PhantomData;

const DEGREE: usize = 2;

/// Special complete binary merkle tree that is compatible with
/// <https://github.com/o1-labs/snarky/blob/master/src/base/merkle_tree.ml>
/// whose leaf nodes are at the same height
pub struct MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<DEGREE, Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<DEGREE, Hash = THash>,
    THash: Clone,
{
    height: u32,
    leafs: Vec<(TItem, Option<THash>)>,
    nodes: Vec<Option<THash>>,

    _pd_hasher: PhantomData<THasher>,
    _pd_merger: PhantomData<TMerger>,
}

impl<TItem, THash, THasher, TMerger> MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<DEGREE, Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<DEGREE, Hash = THash>,
    THash: Clone,
{
    /// Creates a new instance of [MinaMerkleTree]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new instance of [MinaMerkleTree] with estimated capacity of leaves
    pub fn with_capacity(capacity: usize) -> Self {
        let potential_height = calculate_height(capacity);
        let potential_node_count = calculate_node_count(potential_height);
        Self {
            leafs: Vec::with_capacity(capacity),
            nodes: Vec::with_capacity(potential_node_count),
            ..Default::default()
        }
    }

    /// Clears cached hashes of all ancester nodes of the give leaf
    /// because the values become invaid once the leaf is updated
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

    /// Calucates hash of a node if it's not available in the node cache
    /// either apply hash algorithm if it's a leaf node
    /// or apply merge algorithm if it's a non-leaf node
    /// update the cache once calculated
    fn calculate_hash_if_needed(&mut self, index: usize) -> Option<THash> {
        if index < self.nodes.len() {
            if let Some(hash) = &self.nodes[index] {
                Some(hash.clone())
            } else {
                let left = index * 2 + 1;
                let right = index * 2 + 2;
                let left_hash = self.calculate_hash_if_needed(left);
                let right_hash = self.calculate_hash_if_needed(right);
                let hash = TMerger::merge(
                    [left_hash, right_hash],
                    MerkleTreeNodeMetadata::new(index, self.height),
                );
                self.nodes[index] = hash.clone();
                hash
            }
        } else {
            let leaf_index = index - self.nodes.len();
            if leaf_index < self.leafs.len() {
                let (data, hash) = &mut self.leafs[leaf_index];
                match hash {
                    None => {
                        let node_hash = Some(THasher::hash(
                            data,
                            MerkleTreeNodeMetadata::new(index, self.height),
                        ));
                        *hash = node_hash.clone();
                        node_hash
                    }
                    _ => hash.clone(),
                }
            } else {
                None
            }
        }
    }
}

impl<TItem, THash, THasher, TMerger> MerkleTree<DEGREE>
    for MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<DEGREE, Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<DEGREE, Hash = THash>,
    THash: Clone,
{
    type Item = TItem;
    type Hash = THash;

    fn height(&self) -> u32 {
        self.height
    }

    fn count(&self) -> usize {
        self.leafs.len()
    }

    fn root(&mut self) -> Option<Self::Hash> {
        self.calculate_hash_if_needed(0)
    }

    fn add_batch(&mut self, items: impl IntoIterator<Item = Self::Item>) {
        let mut leaves: Vec<_> = items
            .into_iter()
            .map(|item| {
                (
                    // Tree height might be changed, do not calculate hash here.
                    item, None,
                )
            })
            .collect();
        let new_leaf_count = self.leafs.len() + leaves.len();
        let new_height = calculate_height(new_leaf_count);
        if new_height != self.height {
            let new_node_count = calculate_node_count(new_height);
            self.height = new_height;
            self.nodes = vec![None; new_node_count];
        } else {
            let start = self.nodes.len() + self.leafs.len();
            for i in start..(start + leaves.len()) {
                self.clear_dirty_hashes(i);
            }
        }
        self.leafs.append(&mut leaves);
    }

    fn add(&mut self, item: Self::Item) {
        self.add_batch(vec![item])
    }
}

impl<TItem, THash, THasher, TMerger> Default for MinaMerkleTree<TItem, THash, THasher, TMerger>
where
    THasher: MerkleHasher<DEGREE, Item = TItem, Hash = THash>,
    TMerger: MerkleMerger<DEGREE, Hash = THash>,
    THash: Clone,
{
    fn default() -> Self {
        Self {
            height: 0,
            leafs: Vec::new(),
            nodes: Vec::new(),
            _pd_hasher: Default::default(),
            _pd_merger: Default::default(),
        }
    }
}

fn calculate_height(size: usize) -> u32 {
    if size < 2 {
        0
    } else {
        (size as f64).log2().ceil() as u32
    }
}

fn calculate_node_count(height: u32) -> usize {
    2_usize.pow(height) - 1
}

fn calculate_parent_index(index: usize) -> usize {
    debug_assert!(index > 0);
    (index - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_height_tests() {
        assert_eq!(0, calculate_height(0));
        assert_eq!(0, calculate_height(1));
        assert_eq!(1, calculate_height(2));
        assert_eq!(2, calculate_height(3));
        assert_eq!(2, calculate_height(4));
        assert_eq!(3, calculate_height(5));
        assert_eq!(4, calculate_height(11));
        assert_eq!(5, calculate_height(29));
        // Genesis ledger account number
        assert_eq!(11, calculate_height(1676));
    }
}
