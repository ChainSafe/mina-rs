// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// Test cases are generated from ocaml code
//
// (* utop -require core_kernel *)
// #use "https://github.com/o1-labs/snarky/blob/master/src/base/merkle_tree.ml"
//
// let my_hash x = x

// let my_merge a b =
//   match a with
//   | None -> None
//   | Some a -> ( match b with None -> Some (a * 2) | Some b -> Some (a + b))

// let create_tree n =
//   let tree = create ~hash:my_hash ~merge:my_merge 0 in
//   add_many tree (List.init (n - 1) ~f:(fun i -> i + 1))
//
// create_tree 127 |> root
// create_tree 127 |> depth

#[cfg(test)]
mod tests {
    use mina_merkle::*;

    struct TestHasher {}

    impl MerkleHasher for TestHasher {
        type Item = i64;
        type Hash = i64;
        fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata<2>) -> Self::Hash {
            *item
        }
    }

    struct TestMerger {}

    impl MerkleMerger for TestMerger {
        type Hash = i64;
        fn merge(
            hashes: [Option<Self::Hash>; 2],
            _: MerkleTreeNodeMetadata<2>,
        ) -> Option<Self::Hash> {
            match (hashes[0], hashes[1]) {
                (Some(left), Some(right)) => Some(left + right),
                (Some(left), None) => Some(left * 2),
                _ => None,
            }
        }
    }

    type TestMerkleTree = MinaMerkleTree<
        <TestHasher as MerkleHasher>::Item,
        <TestHasher as MerkleHasher>::Hash,
        TestHasher,
        TestMerger,
    >;

    #[test]
    fn mina_merkle_tree_tests_0() {
        let mut tree = TestMerkleTree::new();
        assert!(tree.root().is_none())
    }

    #[test]
    fn mina_merkle_tree_tests_1() {
        test_mina_merkle_tree(1, 0, 0);
    }

    #[test]
    fn mina_merkle_tree_tests_10() {
        test_mina_merkle_tree(10, 96, 4);
    }

    #[test]
    fn mina_merkle_tree_tests_111() {
        test_mina_merkle_tree(111, 7870, 7);
    }

    #[test]
    fn mina_merkle_tree_tests_127() {
        test_mina_merkle_tree(127, 8127, 7);
    }

    #[test]
    fn mina_merkle_tree_tests_128() {
        test_mina_merkle_tree(128, 8128, 7);
    }

    #[test]
    fn mina_merkle_tree_tests_129() {
        test_mina_merkle_tree(129, 24512, 8);
    }

    #[test]
    fn mina_merkle_tree_tests_188() {
        test_mina_merkle_tree(188, 28512, 8);
    }

    fn test_mina_merkle_tree(n: usize, expected_root_hash: i64, expected_height: u32) {
        let mut tree = TestMerkleTree::with_capacity(n);
        let v: Vec<i64> = (0..n).map(|i| i as i64).collect();
        tree.add_batch(v);

        assert_eq!(tree.count(), n);
        assert_eq!(tree.height(), expected_height);
        assert_eq!(tree.root().unwrap(), expected_root_hash);
    }

    #[test]
    fn mina_merkle_tree_expansion_tests() {
        let mut tree = Default::default();
        test_expand_mina_merkle_tree(&mut tree, 1, 0, 0);
        test_expand_mina_merkle_tree(&mut tree, 9, 92, 4);
        test_expand_mina_merkle_tree(&mut tree, 10, 96, 4);
        test_expand_mina_merkle_tree(&mut tree, 111, 7870, 7);
        test_expand_mina_merkle_tree(&mut tree, 127, 8127, 7);
        test_expand_mina_merkle_tree(&mut tree, 128, 8128, 7);
        test_expand_mina_merkle_tree(&mut tree, 129, 24512, 8);
        test_expand_mina_merkle_tree(&mut tree, 188, 28512, 8);
    }

    fn test_expand_mina_merkle_tree(
        tree: &mut TestMerkleTree,
        n: usize,
        expected_root_hash: i64,
        expected_height: u32,
    ) {
        let v: Vec<i64> = (tree.count()..n).map(|i| i as i64).collect();
        if v.len() > 1 {
            tree.add_batch(v);
        } else if v.len() == 1 {
            tree.add(v[0]);
        }

        assert_eq!(tree.count(), n);
        assert_eq!(tree.height(), expected_height);
        assert_eq!(tree.root().unwrap(), expected_root_hash);
    }
}
