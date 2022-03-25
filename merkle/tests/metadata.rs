// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_merkle::*;

    #[test]
    fn test_metadata_depth() {
        assert_eq!(MerkleTreeNodeMetadata::<2>::new(0).depth(), 0);
        assert_eq!(MerkleTreeNodeMetadata::<2>::new(1).depth(), 1);
        assert_eq!(MerkleTreeNodeMetadata::<2>::new(2).depth(), 1);
        assert_eq!(MerkleTreeNodeMetadata::<2>::new(3).depth(), 2);
        assert_eq!(MerkleTreeNodeMetadata::<2>::new(6).depth(), 2);
        assert_eq!(MerkleTreeNodeMetadata::<2>::new(7).depth(), 3);
    }
}
