// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use mina_serialization_types::json::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn char_json_wasm() {
        char_json().unwrap()
    }

    #[test]
    fn char_json() -> anyhow::Result<()> {
        let json_str = r#""\u0001""#;
        let char: CharJson = serde_json::from_str(json_str)?;
        assert_eq!(char.0, 1);
        let json_str_from_char = serde_json::to_string(&char)?;
        assert_eq!(json_str, json_str_from_char.as_str());
        Ok(())
    }

    #[wasm_bindgen_test]
    fn sok_digest_json_wasm() {
        sok_digest_json().unwrap()
    }

    #[test]
    fn sok_digest_json() -> anyhow::Result<()> {
        let block = test_fixtures::JSON_TEST_BLOCKS
            .get("mainnet-116121-3NK6myZRzc3GvS5iydv88on2XTEU2btYrjMVkgtbuoeXASRipSa6.json")
            .unwrap();
        let json_value = &block["staged_ledger_diff"]["diff"][0]["completed_works"][0]["proofs"][1]
            ["statement"]["sok_digest"];
        let byte_vec: ByteVecJson = serde_json::from_value(json_value.clone())?;
        let json_value_from_byte_vec = serde_json::to_value(&byte_vec)?;
        assert_eq!(json_value, &json_value_from_byte_vec);
        Ok(())
    }
}
