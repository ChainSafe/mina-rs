// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use ark_ff::BigInteger256;
    use mina_hasher::{Hashable, ROInput};
    use mina_merkle::*;
    use num::BigUint;

    #[derive(Debug, Clone)]
    struct TestLeafNode(u64);

    impl Hashable for TestLeafNode {
        type D = ();

        fn to_roinput(&self) -> mina_hasher::ROInput {
            let mut roi = ROInput::new();
            roi.append_u64(self.0);
            roi
        }

        fn domain_string(_: Option<&Self>, _: Self::D) -> Option<String> {
            None
        }
    }

    type TestHasher = MinaPoseidonMerkleHasher<TestLeafNode>;

    type TestMerkleTree = MinaMerkleTree<
        <TestHasher as MerkleHasher>::Item,
        <TestHasher as MerkleHasher>::Hash,
        TestHasher,
        MinaPoseidonMerkleMerger,
    >;

    // TODO: More test cases
    #[test]
    fn mina_merkle_tree_tests_0() {
        let mut tree = TestMerkleTree::new();
        assert!(tree.root().is_none())
    }

    #[test]
    fn mina_merkle_tree_tests_10() {
        test_mina_merkle_tree(
            10,
            "10599279112010691761547841576274298116586865112690084147431279883352819577624",
            4,
        );
    }

    fn test_mina_merkle_tree(n: usize, expected_root_hash_hex: &str, expected_depth: u32) {
        let mut tree = TestMerkleTree::with_capacity(n);
        let v: Vec<_> = (0..n).map(|i| TestLeafNode(i as u64)).collect();
        tree.add_batch(v);

        assert_eq!(tree.count(), n);
        assert_eq!(tree.depth(), expected_depth);
        let root = tree.root().unwrap();
        let big256: BigInteger256 = root.into();
        let big: BigUint = big256.into();
        assert_eq!(big.to_str_radix(10), expected_root_hash_hex);
    }
}
