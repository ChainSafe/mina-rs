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
        test_mina_merkle_tree_tests(1, 0);
    }

    #[test]
    fn mina_merkle_tree_tests_10() {
        test_mina_merkle_tree_tests(10, 45);
    }

    #[test]
    fn mina_merkle_tree_tests_111() {
        test_mina_merkle_tree_tests(111, 6105);
    }

    #[test]
    fn mina_merkle_tree_tests_127() {
        test_mina_merkle_tree_tests(127, 8001);
    }

    #[test]
    fn mina_merkle_tree_tests_128() {
        test_mina_merkle_tree_tests(128, 8128);
    }

    #[test]
    fn mina_merkle_tree_tests_129() {
        test_mina_merkle_tree_tests(129, 8256);
    }

    fn test_mina_merkle_tree_tests(n: usize, expected_root_hash: i64) {
        let mut tree = MinaMerkleTree::<i64, i64, TestHasher, TestMerger>::with_capacity(n);
        let v: Vec<i64> = (0..n).map(|i| i as i64).collect();
        tree.add_batch(v);

        assert_eq!(tree.root().unwrap(), expected_root_hash);
    }
}
