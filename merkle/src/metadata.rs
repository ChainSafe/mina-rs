// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// Metadata of a give tree node, including index and depth
/// in the merkle tree it belongs to, which can be used for
/// calculating hash
#[derive(Debug, Clone, Default)]
pub struct MerkleTreeNodeMetadata<const DEGREE: usize> {
    index: usize,
}

impl<const DEGREE: usize> MerkleTreeNodeMetadata<DEGREE> {
    /// Creates a new instance of [MerkleTreeNodeMetadata] with
    /// its index in the merkle tree
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    /// 0-based Index of the tree node
    pub fn index(&self) -> usize {
        self.index
    }

    /// 0-based depth of the tree node
    pub fn depth(&self) -> u32 {
        ((self.index + DEGREE) as f64).log(DEGREE as f64).ceil() as u32 - 1
    }
}
