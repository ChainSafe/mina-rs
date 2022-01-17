// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, Default)]
pub struct MerkleTreeNodeMetadata {
    index: usize,
}

impl MerkleTreeNodeMetadata {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    /// 0-based depth of a tree node with given 0-based index
    pub fn depth(&self) -> u32 {
        (self.index as f64 + 2.).log2().ceil() as u32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_depth() {
        assert_eq!(MerkleTreeNodeMetadata::new(0).depth(), 0);
        assert_eq!(MerkleTreeNodeMetadata::new(1).depth(), 1);
        assert_eq!(MerkleTreeNodeMetadata::new(2).depth(), 1);
        assert_eq!(MerkleTreeNodeMetadata::new(3).depth(), 2);
        assert_eq!(MerkleTreeNodeMetadata::new(6).depth(), 2);
        assert_eq!(MerkleTreeNodeMetadata::new(7).depth(), 3);
    }
}
