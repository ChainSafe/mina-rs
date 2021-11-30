// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use bin_prot::BinProtDeser;
    use mina_consensus::genesis::{GenesisInit, GenesisInitConfig, DEVNET_CONFIG, MAINNET_CONFIG};
    use mina_rs_base::types::*;
    use pretty_assertions::assert_eq;
    use test_fixtures::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_roundtrip_mainnet() {
        test_genesis_roundtrip(&GENESIS_BLOCK_MAINNET)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_roundtrip_devnet() {
        test_genesis_roundtrip(&GENESIS_BLOCK_DEVNET)
    }

    fn test_genesis_roundtrip(genesis_fixture: &BlockFixture) {
        let genesis = genesis_fixture.external_transition().unwrap();
        let output = genesis.try_serialize().unwrap();
        assert_eq!(genesis_fixture.bytes, output)
    }

    #[ignore = "genesis config for devnet not implemented yet"]
    #[test]
    // #[wasm_bindgen_test]
    fn test_genesis_consensus_mainnet() {
        test_genesis_consensus(&GENESIS_BLOCK_MAINNET, &MAINNET_CONFIG)
    }

    #[ignore = "genesis config for devnet not implemented yet"]
    #[test]
    // #[wasm_bindgen_test]
    fn test_genesis_consensus_devnet() {
        test_genesis_consensus(&GENESIS_BLOCK_DEVNET, &DEVNET_CONFIG)
    }

    fn test_genesis_consensus(
        genesis_fixture: &BlockFixture,
        genesis_init_config: &GenesisInitConfig,
    ) {
        let genesis = ExternalTransition::init_genesis(genesis_init_config);
        let output = genesis.try_serialize().unwrap();
        assert_eq!(genesis_fixture.bytes, output)
    }
}
