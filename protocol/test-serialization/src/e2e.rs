// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_rs_base::types::*;
    use mina_serialization_types::*;
    use pretty_assertions::assert_eq;
    use test_fixtures::TEST_BLOCKS;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_block_serde_roundtrip() {
        for block in TEST_BLOCKS.values() {
            let versioned = block.external_transitionv1().unwrap();

            let et: ExternalTransition = versioned.clone().into();
            let versioned2: <ExternalTransition as BinProtSerializationType>::T = et.into();
            assert_eq!(versioned, versioned2);

            let mut bytes = Vec::with_capacity(block.bytes.len());
            bin_prot::to_writer(&mut bytes, &versioned).unwrap();
            assert_eq!(bytes.as_slice(), block.bytes.as_slice(),);
        }
    }
}
