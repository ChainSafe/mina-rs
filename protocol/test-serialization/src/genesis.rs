// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use super::super::tests::select_path;
    use mina_consensus::genesis::*;
    use mina_crypto::prelude::*;
    use mina_rs_base::network_types::v1::ExternalTransitionV1;
    use mina_rs_base::types::ExternalTransition;
    use pretty_assertions::assert_eq;
    use serde::Serialize;
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
        let output = genesis.try_encode_binprot().unwrap();
        assert_eq!(genesis_fixture.bytes, output)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_protocol_state_proof() {
        for et in [
            ExternalTransition::from_genesis_config(&MAINNET_CONFIG),
            GENESIS_BLOCK_MAINNET.external_transition().unwrap().into(),
        ] {
            let protocol_state_proof = &et.protocol_state_proof;
            let ev0 = &protocol_state_proof.proof.openings.evals.0;
            assert_eq!(
                ev0.l.to_hex_string(),
                "2e53605b801ad7fea745e9766add8da9ed33589d758fb339fed40c329c59aa27"
            );
            assert_eq!(
                ev0.r.to_hex_string(),
                "b77a8788b07f7cd1c9c61618755cca3d0d303a7b096124ce0c02dc5f451a0f03"
            );
            assert_eq!(
                ev0.o.to_hex_string(),
                "2e1e68731d00b84720038823777ec6522d9a1e9e365920c3e7ce064ade0c2e1e"
            );
            assert_eq!(
                ev0.z.to_hex_string(),
                "d96d62e54a0a49d3a44c919eb4b089333d64a236edcda1921274ac6903bad937"
            );
            assert_eq!(ev0.t.0.len(), 5);
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_consensus_mainnet() {
        test_genesis_consensus(&GENESIS_BLOCK_MAINNET, &MAINNET_CONFIG)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_consensus_devnet() {
        test_genesis_consensus(&GENESIS_BLOCK_DEVNET, &DEVNET_CONFIG)
    }

    fn test_genesis_consensus(
        genesis_fixture: &BlockFixture,
        genesis_init_config: &GenesisInitConfig,
    ) {
        let genesis = ExternalTransition::from_genesis_config(genesis_init_config);
        let output = genesis.try_encode_binprot().unwrap();
        assert_eq!(genesis_fixture.bytes, output)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_path_mainnet() {
        test_genesis_path(&MAINNET_CONFIG, &GENESIS_BLOCK_MAINNET)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_path_devnet() {
        test_genesis_path(&DEVNET_CONFIG, &GENESIS_BLOCK_DEVNET)
    }

    fn test_genesis_path(genesis_init_config: &GenesisInitConfig, fixture: &BlockFixture) {
        let genesis = ExternalTransitionV1::from_genesis_config(genesis_init_config);

        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/previous_state_hash",
            |b| &b.0.t.protocol_state.t.t.previous_state_hash,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/genesis_state_hash",
            |b| &b.0.t.protocol_state.t.t.body.t.t.genesis_state_hash,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/blockchain_state",
            |b| &b.0.t.protocol_state.t.t.body.t.t.blockchain_state,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/consensus_state",
            |b| &b.0.t.protocol_state.t.t.body.t.t.consensus_state,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/constants",
            |b| &b.0.t.protocol_state.t.t.body.t.t.constants,
        );
        test_path(&genesis, &fixture, "t/protocol_state", |b| {
            &b.0.t.protocol_state
        });

        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values",
            |b| &b.0.t.protocol_state_proof.statement.proof_state.deferred_values,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/sponge_digest_before_evaluations",
            |b| &b.0.t.protocol_state_proof.statement.proof_state.sponge_digest_before_evaluations,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only",
            |b| &b.0.t.protocol_state_proof.statement.proof_state.me_only,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state",
            |b| &b.0.t.protocol_state_proof.statement.proof_state,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through",
            |b| &b.0.t.protocol_state_proof.statement.pass_through,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement",
            |b| &b.0.t.protocol_state_proof.statement,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/prev_evals",
            |b| &b.0.t.protocol_state_proof.prev_evals,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/prev_x_hat",
            |b| &b.0.t.protocol_state_proof.prev_x_hat,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/messages",
            |b| &b.0.t.protocol_state_proof.proof.messages,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/lr",
            |b| &b.0.t.protocol_state_proof.proof.openings.proof.lr,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof",
            |b| &b.0.t.protocol_state_proof.proof.openings.proof,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals",
            |b| &b.0.t.protocol_state_proof.proof.openings.evals,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings",
            |b| &b.0.t.protocol_state_proof.proof.openings,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof",
            |b| &b.0.t.protocol_state_proof.proof,
        );
        test_path(&genesis, &fixture, "t/protocol_state_proof", |b| {
            &b.0.t.protocol_state_proof
        });

        test_path(&genesis, &fixture, "t/staged_ledger_diff", |b| {
            &b.0.t.staged_ledger_diff
        });

        test_path(&genesis, &fixture, "t/delta_transition_chain_proof", |b| {
            &b.0.t.delta_transition_chain_proof
        });

        test_path(&genesis, &fixture, "t/current_protocol_version", |b| {
            &b.0.t.current_protocol_version
        });

        test_path(&genesis, &fixture, "t/proposed_protocol_version_opt", |b| {
            &b.0.t.proposed_protocol_version_opt
        });
    }

    fn test_path<T>(
        et: &ExternalTransitionV1,
        block_fixture: &BlockFixture,
        path: impl AsRef<str>,
        select: fn(et: &ExternalTransitionV1) -> &T,
    ) where
        T: std::fmt::Debug + PartialEq + Serialize,
    {
        let et_deserialized = block_fixture.external_transition().unwrap();
        test_path_typed(et, &et_deserialized, select);
        let _ = et_deserialized;
        let path = path.as_ref();
        let loosely_typed = select_path(&block_fixture.value, path);
        let mut loosely_typed_bytes = vec![];
        bin_prot::to_writer(&mut loosely_typed_bytes, loosely_typed).unwrap();

        let mut strongly_typed_bytes = vec![];
        bin_prot::to_writer(&mut strongly_typed_bytes, select(et)).unwrap();

        assert_eq!(loosely_typed_bytes, strongly_typed_bytes, "path: {}", path,);
    }

    fn test_path_typed<'a, T>(
        a: &'a ExternalTransitionV1,
        b: &'a ExternalTransitionV1,
        select: fn(et: &'a ExternalTransitionV1) -> &'a T,
    ) where
        T: std::fmt::Debug + PartialEq,
    {
        assert_eq!(select(a), select(b))
    }
}
