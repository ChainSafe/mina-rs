// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use std::{cmp::Ordering, marker::PhantomData};

// modes of tree operation

/// Type state mode for a tree
pub trait HeightMode {
    /// Gets the fixed height if applicable
    fn fixed_height(&self) -> Option<u32>;
}

#[derive(Default, Debug, Clone)]
/// Type state mode for a tree with a fixed height
pub struct FixedHeightMode(u32);

impl HeightMode for FixedHeightMode {
    fn fixed_height(&self) -> Option<u32> {
        Some(self.0)
    }
}

#[derive(Default, Debug, Clone)]
/// Type state mode for a tree with a variable height that increases as data is added
pub struct VariableHeightMode;

impl HeightMode for VariableHeightMode {
    fn fixed_height(&self) -> Option<u32> {
        None
    }
}

/// Special complete binary merkle tree that is compatible with
/// <https://github.com/o1-labs/snarky/blob/master/src/base/merkle_tree.ml>
/// whose leaf nodes are at the same height
pub struct MinaMerkleTree<Item, Hash, Hasher, Merger, Mode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
    Mode: HeightMode,
{
    mode: Mode,
    variable_height: u32,
    leafs: Vec<(Item, Option<Hash>)>,
    nodes: Vec<Option<Hash>>,

    _pd_hasher: PhantomData<Hasher>,
    _pd_merger: PhantomData<Merger>,
}

impl<Item, Hash, Hasher, Merger> MinaMerkleTree<Item, Hash, Hasher, Merger, FixedHeightMode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
{
    /// Creates a new instance of a fixed height MinaMerkleTree
    pub fn new(height: u32) -> Self {
        Self {
            mode: FixedHeightMode(height),
            ..Default::default()
        }
    }
}

impl<Item, Hash, Hasher, Merger> MinaMerkleTree<Item, Hash, Hasher, Merger, VariableHeightMode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
{
    /// Creates a new instance of a variable height MinaMerkletree
    pub fn new() -> Self {
        Default::default()
    }
}

impl<Item, Hash, Hasher, Merger, Mode> MinaMerkleTree<Item, Hash, Hasher, Merger, Mode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
    Mode: Default + HeightMode,
{
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

    /// Gets the merkle proof of an item with the 0-based index of the item
    /// being added, e.g. the first item is index 0.
    /// This function panics when the index is out of range.
    pub fn get_proof(
        &mut self,
        index: usize,
    ) -> Option<DefaultMerkleProof<Item, Hash, Hasher, Merger>> {
        if self.calculate_hash_if_needed(0).is_some() {
            // 1. Calculates the number of nodes above the data node,
            // for calculating its index in the tree (0 for the root, counted from topdown, left to right)
            let height_above = if let Some(fixed_height) = self.mode.fixed_height() {
                fixed_height
            } else {
                self.variable_height
            };
            let index_offset = calculate_node_count(height_above);
            let capacity = self.mode.fixed_height().unwrap_or(self.variable_height) as usize;
            let mut peer_indices = Vec::with_capacity(capacity);
            let mut peer_hashes = Vec::with_capacity(capacity);
            let (item, _) = &self.leafs[index];
            let index_with_offset = index_offset + index;
            // 2. Gets the index and hash of its sibling in the tree, and push to the proof vec
            let peer_index = if index % 2 == 0 { index + 1 } else { index - 1 };
            let peer_index_with_offset = index_offset + peer_index;
            let peer_hash = if peer_index < self.leafs.len() {
                self.leafs[peer_index].1.clone()
            } else {
                None
            };
            peer_indices.push(peer_index_with_offset);
            peer_hashes.push(peer_hash);
            // 3. Gets the index of their parent node and point the cursor to it
            // parent_index is the index of the actual tree with the variable height
            let mut parent_index = calculate_parent_index(self.nodes.len() + index);
            // and parent_index_with_offset is the index of the virtual tree with possible fixed height
            // when the fixed height equals to the variable height, parent_index equals to parent_index_with_offset
            let mut parent_index_with_offset = calculate_parent_index(index_with_offset);
            while parent_index_with_offset > 0 {
                if parent_index > 0 {
                    // 4. Gets the index and hash of the sibling of this node, and push to the proof vec
                    let parent_peer_index = if parent_index % 2 == 0 {
                        parent_index - 1
                    } else {
                        parent_index + 1
                    };
                    let parent_peer_index_with_offset =
                        parent_index_with_offset + parent_peer_index - parent_index;
                    peer_indices.push(parent_peer_index_with_offset);
                    peer_hashes.push(self.nodes[parent_peer_index].clone());
                    parent_index = calculate_parent_index(parent_index);
                } else {
                    // 4.1 When it comes to virtual nodes(only when fixed node count > variable node count)
                    // that are not stored in the tree, use None hash and leave it for the merger to recalculate
                    // in proof verification flow
                    peer_indices.push(parent_index_with_offset + 1);
                    peer_hashes.push(None);
                }
                // 5. Go back to step 3, point the cursor to its parent and apply the same flow
                // until root node is hit
                parent_index_with_offset = calculate_parent_index(parent_index_with_offset);
            }
            Some(DefaultMerkleProof::new(
                index_with_offset,
                item.clone(),
                peer_indices,
                peer_hashes,
            ))
        } else {
            None
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
    fn calculate_hash_if_needed(&mut self, index: usize) -> Option<Hash> {
        if index < self.nodes.len() {
            if let Some(hash) = &self.nodes[index] {
                Some(hash.clone())
            } else {
                let left = index * 2 + 1;
                let right = index * 2 + 2;
                let left_hash = self.calculate_hash_if_needed(left);
                let right_hash = self.calculate_hash_if_needed(right);
                let hash = Merger::merge(
                    [left_hash, right_hash],
                    MerkleTreeNodeMetadata::new(index, self.variable_height),
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
                        let node_hash = Some(Hasher::hash(
                            data,
                            MerkleTreeNodeMetadata::new(index, self.variable_height),
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

impl<Item, Hash, Hasher, Merger> MerkleTree
    for MinaMerkleTree<Item, Hash, Hasher, Merger, VariableHeightMode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
{
    type Item = Item;
    type Hash = Hash;

    fn height(&self) -> u32 {
        self.variable_height
    }

    fn count(&self) -> usize {
        self.leafs.len()
    }

    fn root(&mut self) -> Option<Self::Hash> {
        self.calculate_hash_if_needed(0)
    }

    fn add_batch(&mut self, items: impl IntoIterator<Item = Self::Item>) {
        add_batch(self, items)
    }
}

impl<Item, Hash, Hasher, Merger> MerkleTree
    for MinaMerkleTree<Item, Hash, Hasher, Merger, FixedHeightMode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
{
    type Item = Item;
    type Hash = Hash;

    fn height(&self) -> u32 {
        self.mode.0
    }

    fn count(&self) -> usize {
        self.leafs.len()
    }

    fn root(&mut self) -> Option<Self::Hash> {
        let mut hash = self.calculate_hash_if_needed(0);
        let fixed_height = self.mode.0;
        match fixed_height.cmp(&self.variable_height) {
            Ordering::Less => panic!(
                "fixed_height {fixed_height} should not be smaller than current height {}",
                self.variable_height,
            ),
            Ordering::Equal => hash,
            Ordering::Greater => {
                for h in (self.variable_height + 1)..=fixed_height {
                    hash = Merger::merge([hash, None], MerkleTreeNodeMetadata::new(0, h));
                }
                hash
            }
        }
    }

    fn add_batch(&mut self, items: impl IntoIterator<Item = Self::Item>) {
        add_batch(self, items)
    }
}

impl<Item, Hash, Hasher, Merger, Mode> Default for MinaMerkleTree<Item, Hash, Hasher, Merger, Mode>
where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
    Mode: Default + HeightMode,
{
    fn default() -> Self {
        Self {
            mode: Default::default(),
            variable_height: 0,
            leafs: Vec::new(),
            nodes: Vec::new(),
            _pd_hasher: Default::default(),
            _pd_merger: Default::default(),
        }
    }
}

fn add_batch<Item, Hash, Hasher, Merger, Mode>(
    tree: &mut MinaMerkleTree<Item, Hash, Hasher, Merger, Mode>,
    items: impl IntoIterator<Item = Item>,
) where
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
    Hash: Clone + PartialEq + std::fmt::Debug,
    Item: Clone,
    Mode: Default + HeightMode,
{
    let mut leaves: Vec<_> = items
        .into_iter()
        .map(|item| {
            (
                // Tree height might be changed, do not calculate hash here.
                item, None,
            )
        })
        .collect();
    let new_leaf_count = tree.leafs.len() + leaves.len();
    let new_height = calculate_height(new_leaf_count);
    if new_height != tree.variable_height {
        let new_node_count = calculate_node_count(new_height);
        tree.variable_height = new_height;
        tree.nodes = vec![None; new_node_count];
    } else {
        let start = tree.nodes.len() + tree.leafs.len();
        for i in start..(start + leaves.len()) {
            tree.clear_dirty_hashes(i);
        }
    }
    tree.leafs.append(&mut leaves);
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
