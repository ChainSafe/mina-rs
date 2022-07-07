// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use mina_rs_base::types::*;
    use mina_serialization_types::*;
    use pretty_assertions::assert_eq;
    use test_fixtures::TEST_BLOCKS;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_block_state_hash() {
        for (file_name, block_fixture) in test_fixtures::TEST_BLOCKS.iter() {
            let expected_state_hash = file_name.split('.').nth(0).unwrap_or_default();
            if !expected_state_hash.contains("block") {
                let block_v1 = block_fixture.external_transitionv1().unwrap();
                let block: ExternalTransition = block_v1.into();
                let state_hash = block.protocol_state.state_hash();
                assert_eq!(&state_hash.to_string(), expected_state_hash);
            }
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_block_serde_roundtrip() {
        for block in TEST_BLOCKS.values() {
            let versioned = block.external_transitionv1().unwrap();

            let et: ExternalTransition = versioned.clone().into();
            let et2: ExternalTransition =
                <ExternalTransition as BinProtSerializationType>::try_from_binprot(
                    block.bytes.as_slice(),
                )
                .unwrap();
            assert_eq!(et, et2);
            let versioned2: <ExternalTransition as BinProtSerializationType>::T = et.clone().into();
            assert_eq!(versioned, versioned2);

            let bytes = et.try_into_binprot().unwrap();
            assert_eq!(bytes.as_slice(), block.bytes.as_slice());
        }
    }
}
