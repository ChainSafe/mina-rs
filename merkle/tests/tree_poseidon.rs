// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use proof_systems::*;

    use ark_ff::{BigInteger256, FromBytes};
    use mina_hasher::{Fp, Hashable, ROInput};
    use mina_merkle::*;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct TestLeafNode(Fp);

    impl Hashable for TestLeafNode {
        type D = ();

        fn to_roinput(&self) -> mina_hasher::ROInput {
            let mut roi = ROInput::new();
            roi.append_field(self.0);
            roi
        }

        fn domain_string(_: Self::D) -> Option<String> {
            None
        }
    }

    struct TestHasher;

    impl MerkleHasher for TestHasher {
        type Item = TestLeafNode;
        type Hash = Fp;
        fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata) -> Self::Hash {
            item.0
        }
    }

    type TestMerkleTree = MinaMerkleTree<
        <TestHasher as MerkleHasher>::Item,
        <TestHasher as MerkleHasher>::Hash,
        TestHasher,
        MinaPoseidonMerkleMerger,
        VariableHeightMode,
    >;

    type TestFixedHeightMerkleTree = MinaMerkleTree<
        <TestHasher as MerkleHasher>::Item,
        <TestHasher as MerkleHasher>::Hash,
        TestHasher,
        MinaPoseidonMerkleMerger,
        FixedHeightMode,
    >;

    // The test case is from genesis ledger
    #[test]
    fn test_mina_poseidon_merkle_merger_even() {
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
        for _ in 0..GENESIS_LEDGER_ACCOUNT_NUM {
            tree.add(TestLeafNode(Fp::default()))
        }
        let meta = MerkleTreeNodeMetadata::new(node_index, tree.height());
        let merged =
            MinaPoseidonMerkleMerger::merge([Some(h1), Some(h2)], meta).unwrap_or_default();
        assert_eq!(h3, merged);
    }

    #[test]
    fn test_mina_poseidon_merkle_merger_odd() {
        let h1: Fp = BigInteger256::read(
            [
                146, 128, 118, 98, 200, 74, 91, 121, 164, 107, 160, 240, 39, 236, 147, 102, 92,
                114, 200, 62, 92, 15, 190, 134, 196, 11, 200, 126, 35, 56, 168, 59,
            ]
            .as_slice(),
        )
        .unwrap()
        .into();
        let h2: Fp = BigInteger256::read(
            [
                181, 78, 220, 55, 96, 63, 185, 35, 72, 225, 102, 60, 60, 81, 94, 219, 35, 27, 49,
                124, 235, 72, 211, 61, 223, 235, 94, 79, 29, 22, 102, 15,
            ]
            .as_slice(),
        )
        .unwrap()
        .into();
        let meta = MerkleTreeNodeMetadata::new(0, 12);
        let merged = MinaPoseidonMerkleMerger::merge([Some(h1), None], meta).unwrap_or_default();
        assert_eq!(h2, merged);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn mina_merkle_tree_genesis_ledger_parity_test() {
        use rocksdb::*;

        let fixed_height = 20;
        let mut merkle_ledger = TestFixedHeightMerkleTree::new(fixed_height);

        // TODO: Use API from DbBackedGenesisLedger to iterate over hash nodes
        let db =
            DB::open_for_read_only(&Options::default(), "../ledger/test-data/genesis_ledger_6a887ea130e53b06380a9ab27b327468d28d4ce47515a0cc59759d4a3912f0ef/", true).unwrap();
        let mut root_height = 0;
        let mut expected_root_hash: Option<Fp> = None;
        for (key, value) in db
            .iterator(IteratorMode::Start)
            .take_while(|(key, _)| key[0] < 0xfe)
        {
            let height = key[0];
            let hash: Fp = BigInteger256::read(&value[2..]).unwrap().into();
            if height > root_height {
                root_height = height;
                expected_root_hash = Some(hash);
            }
            if height == 0 {
                let node = TestLeafNode(hash);
                assert_eq!(
                    hash,
                    TestHasher::hash(&node, MerkleTreeNodeMetadata::new(0, 1))
                );
                merkle_ledger.add(node);
            }
        }
        assert!(expected_root_hash.is_some());
        assert_eq!(fixed_height, root_height as u32);
        assert_eq!(merkle_ledger.root(), expected_root_hash);

        // Test merkle proofs
        let root_hash = &expected_root_hash.unwrap();
        for i in (0..10).chain(merkle_ledger.count() - 10..merkle_ledger.count()) {
            let proof = merkle_ledger.get_proof(i).unwrap();
            assert!(proof.verify(root_hash));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn genesis_ledger_parity_test() {
        use rocksdb::*;

        // TODO: Use API from DbBackedGenesisLedger to iterate over hash nodes
        let db =
            DB::open_for_read_only(&Options::default(), "../ledger/test-data/genesis_ledger_6a887ea130e53b06380a9ab27b327468d28d4ce47515a0cc59759d4a3912f0ef/", true).unwrap();
        let mut height_2_nodes: HashMap<u8, Vec<Fp>> = HashMap::new();
        let mut max_height = 0;
        for (height, hash) in db
            .iterator(IteratorMode::Start)
            .take_while(|(key, _)| key[0] < 0xfe)
            .map(|(key, value)| {
                let height = key[0];
                let hash: Fp = BigInteger256::read(&value[2..]).unwrap().into();
                (height, hash)
            })
        {
            if height > max_height {
                max_height = height;
            }
            if let Some(vec) = height_2_nodes.get_mut(&height) {
                vec.push(hash);
            } else {
                height_2_nodes.insert(height, vec![hash]);
            }
        }
        let mut assert_hit = false;
        for height in (1..max_height).rev() {
            let this_level = height_2_nodes.get(&height).unwrap();
            let next_level = height_2_nodes.get(&(height - 1)).unwrap();
            let next_level_len = next_level.len();
            for (i, hash) in this_level.iter().enumerate() {
                let left = match 2 * i < next_level_len {
                    true => Some(next_level[2 * i]),
                    _ => None,
                };
                let right = match 2 * i + 1 < next_level_len {
                    true => Some(next_level[2 * i + 1]),
                    _ => None,
                };
                // Index is 0 because this is the root node the subtree
                let meta = MerkleTreeNodeMetadata::new(0, height as u32);
                let merged =
                    MinaPoseidonMerkleMerger::merge([left, right], meta).unwrap_or_default();
                assert_eq!(hash, &merged, "fail at height {height}, i {i}");
                assert_hit = true;
            }
        }
        assert!(assert_hit);
    }

    #[test]
    fn mina_merkle_tree_tests_0() {
        let mut tree = TestMerkleTree::new();
        assert!(tree.root().is_none());
    }
}
