// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_merkle::*;

    #[test]
    fn test_metadata_depth() {
        assert_eq!(MerkleTreeNodeMetadata::new(0, 10).depth(), 0);
        assert_eq!(MerkleTreeNodeMetadata::new(1, 10).depth(), 1);
        assert_eq!(MerkleTreeNodeMetadata::new(2, 10).depth(), 1);
        assert_eq!(MerkleTreeNodeMetadata::new(3, 10).depth(), 2);
        assert_eq!(MerkleTreeNodeMetadata::new(6, 10).depth(), 2);
        assert_eq!(MerkleTreeNodeMetadata::new(7, 10).depth(), 3);
        assert_eq!(MerkleTreeNodeMetadata::new(1028, 10).depth(), 10);
    }

    #[test]
    fn test_metadata_height() {
        assert_eq!(MerkleTreeNodeMetadata::new(0, 10).height(), 10);
        assert_eq!(MerkleTreeNodeMetadata::new(1, 10).height(), 9);
        assert_eq!(MerkleTreeNodeMetadata::new(2, 10).height(), 9);
        assert_eq!(MerkleTreeNodeMetadata::new(3, 10).height(), 8);
        assert_eq!(MerkleTreeNodeMetadata::new(6, 10).height(), 8);
        assert_eq!(MerkleTreeNodeMetadata::new(7, 10).height(), 7);
        assert_eq!(MerkleTreeNodeMetadata::new(1028, 10).height(), 0);
    }
}
