// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_merkle::prefixes::*;

    #[test]
    fn make_merkle_tree_hash_3() {
        let prefix_at_3 = make_prefix_merkle_tree(3);
        assert_eq!(&prefix_at_3, "CodaMklTree003");
    }

    #[test]
    fn make_merkle_tree_hash_13() {
        let prefix_at_3 = make_prefix_merkle_tree(13);
        assert_eq!(&prefix_at_3, "CodaMklTree013");
    }

    #[test]
    fn make_merkle_tree_hash_113() {
        let prefix_at_3 = make_prefix_merkle_tree(113);
        assert_eq!(&prefix_at_3, "CodaMklTree113");
    }

    #[test]
    fn make_coinbase_merkle_tree_hash() {
        let prefix_at_3 = make_prefix_coinbase_merkle_tree(3);
        assert_eq!(&prefix_at_3, "CodaCbMklTree003");
    }
}
