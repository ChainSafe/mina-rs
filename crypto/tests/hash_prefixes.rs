// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_crypto::hash::prefixes::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn create_works_as_expected() {
        assert_eq!(PROTOCOL_STATE.len(), 20);
        assert_eq!(PROTOCOL_STATE, b"CodaProtoState******");
    }

    #[test]
    #[wasm_bindgen_test]
    fn make_merkle_tree_hash_3() {
        let prefix_at_3 = make_prefix_merkle_tree(3);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaMklTree003******");
    }

    #[test]
    #[wasm_bindgen_test]
    fn make_merkle_tree_hash_13() {
        let prefix_at_3 = make_prefix_merkle_tree(13);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaMklTree013******");
    }

    #[test]
    #[wasm_bindgen_test]
    fn make_merkle_tree_hash_113() {
        let prefix_at_3 = make_prefix_merkle_tree(113);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaMklTree113******");
    }

    #[test]
    #[wasm_bindgen_test]
    fn make_coinbase_merkle_tree_hash() {
        let prefix_at_3 = make_prefix_coinbase_merkle_tree(3);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(&prefix_at_3, b"CodaCbMklTree003****");
    }
}
