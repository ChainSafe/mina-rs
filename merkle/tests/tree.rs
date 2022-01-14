// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// Test cases are generated from ocaml code
//
// #use "https://github.com/o1-labs/snarky/blob/master/src/base/merkle_tree.ml"
//
// let my_hash x = x

// let my_merge a b =
//   match a with
//   | None -> None
//   | Some a -> ( match b with None -> Some a | Some b -> Some (a + b))

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
        fn hash(item: &Self::Item) -> Self::Hash {
            *item
        }
    }

    struct TestMerger {}

    impl MerkleMerger for TestMerger {
        type Hash = i64;
        fn merge(left: &Option<Self::Hash>, right: &Option<Self::Hash>) -> Option<Self::Hash> {
            if let Some(left) = left {
                if let Some(right) = right {
                    Some(left + right)
                } else {
                    Some(*left)
                }
            } else {
                None
            }
        }
    }

    #[test]
    fn mina_merkle_tree_tests_0() {
        let mut tree = MinaMerkleTree::<i64, i64, TestHasher, TestMerger>::new();
        assert!(tree.root().is_none())
    }

    #[test]
    fn mina_merkle_tree_tests_1() {
        test_mina_merkle_tree(1, 0, 0);
    }

    #[test]
    fn mina_merkle_tree_tests_10() {
        test_mina_merkle_tree(10, 45, 4);
    }

    #[test]
    fn mina_merkle_tree_tests_111() {
        test_mina_merkle_tree(111, 6105, 7);
    }

    #[test]
    fn mina_merkle_tree_tests_127() {
        test_mina_merkle_tree(127, 8001, 7);
    }

    #[test]
    fn mina_merkle_tree_tests_128() {
        test_mina_merkle_tree(128, 8128, 7);
    }

    #[test]
    fn mina_merkle_tree_tests_129() {
        test_mina_merkle_tree(129, 8256, 8);
    }

    #[test]
    fn mina_merkle_tree_tests_188() {
        test_mina_merkle_tree(188, 17578, 8);
    }

    fn test_mina_merkle_tree(n: usize, expected_root_hash: i64, expected_depth: u32) {
        let mut tree = MinaMerkleTree::<i64, i64, TestHasher, TestMerger>::with_capacity(n);
        let v: Vec<i64> = (0..n).map(|i| i as i64).collect();
        tree.add_batch(v);

        assert_eq!(tree.count(), n);
        assert_eq!(tree.depth(), expected_depth);
        assert_eq!(tree.root().unwrap(), expected_root_hash);
    }

    #[test]
    fn mina_merkle_tree_expansion_tests() {
        let mut tree = Default::default();
        test_expand_mina_merkle_tree(&mut tree, 1, 0, 0);
        test_expand_mina_merkle_tree(&mut tree, 9, 36, 4);
        test_expand_mina_merkle_tree(&mut tree, 10, 45, 4);
        test_expand_mina_merkle_tree(&mut tree, 111, 6105, 7);
        test_expand_mina_merkle_tree(&mut tree, 127, 8001, 7);
        test_expand_mina_merkle_tree(&mut tree, 128, 8128, 7);
        test_expand_mina_merkle_tree(&mut tree, 129, 8256, 8);
        test_expand_mina_merkle_tree(&mut tree, 188, 17578, 8);
    }

    fn test_expand_mina_merkle_tree(
        tree: &mut MinaMerkleTree<i64, i64, TestHasher, TestMerger>,
        n: usize,
        expected_root_hash: i64,
        expected_depth: u32,
    ) {
        let v: Vec<i64> = (tree.count()..n).map(|i| i as i64).collect();
        if v.len() > 1 {
            tree.add_batch(v);
        } else if v.len() == 1 {
            tree.add(v[0]);
        }

        assert_eq!(tree.count(), n);
        assert_eq!(tree.depth(), expected_depth);
        assert_eq!(tree.root().unwrap(), expected_root_hash);
    }
}
