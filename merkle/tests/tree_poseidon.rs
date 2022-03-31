// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use ark_ff::{BigInteger256, FromBytes};
    use mina_hasher::{Fp, Hashable, ROInput};
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

        fn domain_string(_: Self::D) -> Option<String> {
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

    // The test case is from genesis ledger
    // TODO: and e2e test to verify every hash node from genesis ledger
    // once https://github.com/ChainSafe/mina-rs/pull/183 is merged
    #[test]
    fn test_mina_poseidon_merkle_merger() {
        let h1: Fp = BigInteger256::read(
            [
                46, 111, 163, 222, 5, 13, 158, 61, 44, 42, 248, 84, 17, 204, 170, 242, 152, 233,
                200, 16, 116, 180, 10, 25, 151, 67, 118, 244, 69, 231, 109, 46,
            ]
            .as_slice(),
        )
        .unwrap()
        .into();
        let h2: Fp = BigInteger256::read(
            [
                145, 28, 178, 157, 78, 192, 208, 17, 235, 147, 39, 0, 105, 97, 173, 134, 12, 183,
                159, 53, 66, 254, 56, 82, 142, 108, 173, 182, 106, 220, 226, 6,
            ]
            .as_slice(),
        )
        .unwrap()
        .into();
        let h3: Fp = BigInteger256::read(
            [
                146, 128, 118, 98, 200, 74, 91, 121, 164, 107, 160, 240, 39, 236, 147, 102, 92,
                114, 200, 62, 92, 15, 190, 134, 196, 11, 200, 126, 35, 56, 168, 59,
            ]
            .as_slice(),
        )
        .unwrap()
        .into();
        // Index of the root node is 0
        let node_index = 0;
        // Genesis ledger has 1676 accounts
        const GENESIS_LEDGER_ACCOUNT_NUM: u64 = 1676;
        let mut tree = TestMerkleTree::new();
        for i in 0..GENESIS_LEDGER_ACCOUNT_NUM {
            tree.add(TestLeafNode(i))
        }
        let meta = MerkleTreeNodeMetadata::new(node_index, tree.height());
        let expected =
            MinaPoseidonMerkleMerger::merge([Some(h1), Some(h2)], meta).unwrap_or_default();
        assert_eq!(expected, h3);
    }

    // TODO: More test cases
    #[test]
    fn mina_merkle_tree_tests_0() {
        let mut tree = TestMerkleTree::new();
        assert!(tree.root().is_none())
    }
}
