// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use bin_prot::Deserializer;
use mina_serialization_types::staged_ledger_diff::{CoinBase, CoinBaseV1};
use proof_systems::mina_signer::CompressedPubKey;
use serde::Deserialize;
use wasm_bindgen_test::*;

const COINBASE_TWO_HEX_EXAMPLES: &[&str] = &[
    // Case 1: Two(Some(ft),None) where 
    // ft: receiver_pk = B62qiTKpEPjGTSHZrtM8uXiKgn8So916pLmNJKDhKeyBQL9TDb3nvBG, fee = 247
    "010201010101010000000000000000000000000000000000000000000000000000000000000000000101fef70000",
    // Case 2: Two(Some(ft1),Some(ft2)) where 
    // ft1: receiver_pk = B62qiTKpEPjGTSHZrtM8uXiKgn8So916pLmNJKDhKeyBQL9TDb3nvBG, fee = 247
    // ft2: receiver_pk = B62qiTKpEPjGTSHZrtM8uXiKgn8So916pLmNJKDhKeyBQL9TDb3nvBG, fee = 1000002
    "010201010101010000000000000000000000000000000000000000000000000000000000000000000101fef70001010101010000000000000000000000000000000000000000000000000000000000000000000101fd42420f00"
];

#[test]
#[wasm_bindgen_test]
fn test_coinbase_two_deserialization() {
    for hex_str in COINBASE_TWO_HEX_EXAMPLES {
        let bytes = hex::decode(hex_str).expect("Failed to decode hex encoded block");
        let mut de = Deserializer::from_reader(bytes.as_slice());
        let result: CoinBaseV1 = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        match result.t {
            // assert Case 1: Two(Some(ft),None)
            CoinBase::Two(Some(ft), None) => {
                let key = CompressedPubKey::from(ft.t.t.receiver_pk.clone()).into_address();
                let fee = ft.t.t.fee.t.t;
                assert_eq!(
                    key,
                    "B62qiTKpEPjGTSHZrtM8uXiKgn8So916pLmNJKDhKeyBQL9TDb3nvBG"
                );
                assert_eq!(fee, 247);
            }
            // assert Case 2: Two(Some(ft1),Some(ft2))
            CoinBase::Two(Some(ft1), Some(ft2)) => {
                let key1 = CompressedPubKey::from(ft1.t.t.receiver_pk.clone()).into_address();
                let fee1 = ft1.t.t.fee.t.t;
                assert_eq!(
                    key1,
                    "B62qiTKpEPjGTSHZrtM8uXiKgn8So916pLmNJKDhKeyBQL9TDb3nvBG"
                );
                assert_eq!(fee1, 247);
                let key2 = CompressedPubKey::from(ft2.t.t.receiver_pk.clone()).into_address();
                let fee2 = ft2.t.t.fee.t.t;
                assert_eq!(
                    key2,
                    "B62qiTKpEPjGTSHZrtM8uXiKgn8So916pLmNJKDhKeyBQL9TDb3nvBG"
                );
                assert_eq!(fee2, 1000002);
            }
            _ => {
                panic!("Unexpected variant!")
            }
        }
    }
}
