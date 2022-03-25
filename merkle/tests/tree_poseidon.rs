// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_hasher::{Hashable, ROInput};
    use mina_merkle::*;

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
}
