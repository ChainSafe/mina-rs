// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// Metadata of a give tree node, including index and depth
/// in the merkle tree it belongs to, which can be used for
/// calculating hash
#[derive(Debug, Clone, Default)]
pub struct MerkleTreeNodeMetadata<const DEGREE: usize> {
    index: usize,
    depth: u32,
    height: u32,
}

impl<const DEGREE: usize> MerkleTreeNodeMetadata<DEGREE> {
    /// Creates a new instance of [MerkleTreeNodeMetadata] with
    /// the node's index in the merkle tree, index of root node is 0
    pub fn new(node_index: usize, tree_height: u32) -> Self {
        let depth = depth::<DEGREE>(node_index);
        let height = height::<DEGREE>(node_index, tree_height);
        Self {
            index: node_index,
            depth,
            height,
        }
    }

    /// 0-based Index of the tree node, starting from root
    pub fn index(&self) -> usize {
        self.index
    }

    /// distance to the root node
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// distance to the leaf nodes that store data
    pub fn height(&self) -> u32 {
        self.height
    }
}

fn depth<const DEGREE: usize>(index: usize) -> u32 {
    ((index + DEGREE) as f64).log(DEGREE as f64).ceil() as u32 - 1
}

fn height<const DEGREE: usize>(index: usize, height: u32) -> u32 {
    height - depth::<DEGREE>(index)
}
