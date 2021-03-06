// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use super::super::tests::select_path;
    use mina_consensus::genesis::*;
    use mina_crypto::prelude::*;
    use mina_rs_base::types::*;
    use mina_serialization_types::v1::ExternalTransitionV1;
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
        let genesis = genesis_fixture.external_transitionv1().unwrap();
        let mut output: Vec<u8> = Vec::new();
        bin_prot::to_writer(&mut output, &genesis).unwrap();
        assert_eq!(genesis_fixture.bytes, output)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_genesis_protocol_state_proof() {
        for et in [
            ExternalTransitionV1::from(ExternalTransition::from_genesis_config(&MAINNET_CONFIG)),
            GENESIS_BLOCK_MAINNET.external_transitionv1().unwrap(),
        ] {
            let protocol_state_proof = &et.t.protocol_state_proof;
            let ev0: ProofEvaluations = protocol_state_proof
                .t
                .t
                .t
                .t
                .proof
                .t
                .t
                .openings
                .t
                .evals
                .0
                .clone()
                .into();
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
        let genesis: ExternalTransitionV1 =
            ExternalTransition::from_genesis_config(genesis_init_config).into();
        let mut output: Vec<u8> = Vec::new();
        bin_prot::to_writer(&mut output, &genesis).unwrap();
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
        let genesis: ExternalTransitionV1 =
            ExternalTransition::from_genesis_config(genesis_init_config).into();

        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/previous_state_hash",
            |b| &b.t.protocol_state.t.t.previous_state_hash,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/genesis_state_hash",
            |b| &b.t.protocol_state.t.t.body.t.t.genesis_state_hash,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/blockchain_state",
            |b| &b.t.protocol_state.t.t.body.t.t.blockchain_state,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/consensus_state",
            |b| &b.t.protocol_state.t.t.body.t.t.consensus_state,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state/t/t/body/t/t/constants",
            |b| &b.t.protocol_state.t.t.body.t.t.constants,
        );
        test_path(&genesis, &fixture, "t/protocol_state", |b| {
            &b.t.protocol_state
        });

        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values",
            |b| {
                &b.t.protocol_state_proof
                    .t
                    .t
                    .t
                    .t
                    .statement
                    .t
                    .t
                    .proof_state
                    .t
                    .deferred_values
            },
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/sponge_digest_before_evaluations",
            |b| &b.t.protocol_state_proof.t.t.t.t.statement.t.t.proof_state.t.sponge_digest_before_evaluations,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only",
            |b| {
                &b.t.protocol_state_proof
                    .t
                    .t
                    .t
                    .t
                    .statement
                    .t
                    .t
                    .proof_state
                    .t
                    .me_only
            },
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state",
            |b| &b.t.protocol_state_proof.t.t.t.t.statement.t.t.proof_state,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through",
            |b| &b.t.protocol_state_proof.t.t.t.t.statement.t.t.pass_through,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/statement",
            |b| &b.t.protocol_state_proof.t.t.t.t.statement,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/prev_evals",
            |b| &b.t.protocol_state_proof.t.t.t.t.prev_evals,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/prev_x_hat",
            |b| &b.t.protocol_state_proof.t.t.t.t.prev_x_hat,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/messages",
            |b| &b.t.protocol_state_proof.t.t.t.t.proof.t.t.messages,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/lr",
            |b| {
                &b.t.protocol_state_proof
                    .t
                    .t
                    .t
                    .t
                    .proof
                    .t
                    .t
                    .openings
                    .t
                    .proof
                    .t
                    .lr
            },
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof",
            |b| &b.t.protocol_state_proof.t.t.t.t.proof.t.t.openings.t.proof,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals",
            |b| &b.t.protocol_state_proof.t.t.t.t.proof.t.t.openings.t.evals,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof/t/t/openings",
            |b| &b.t.protocol_state_proof.t.t.t.t.proof.t.t.openings,
        );
        test_path(
            &genesis,
            &fixture,
            "t/protocol_state_proof/t/t/t/t/proof",
            |b| &b.t.protocol_state_proof.t.t.t.t.proof,
        );
        test_path(&genesis, &fixture, "t/protocol_state_proof", |b| {
            &b.t.protocol_state_proof
        });

        test_path(&genesis, &fixture, "t/staged_ledger_diff", |b| {
            &b.t.staged_ledger_diff
        });

        test_path(&genesis, &fixture, "t/delta_transition_chain_proof", |b| {
            &b.t.delta_transition_chain_proof
        });

        test_path(&genesis, &fixture, "t/current_protocol_version", |b| {
            &b.t.current_protocol_version
        });

        test_path(&genesis, &fixture, "t/proposed_protocol_version_opt", |b| {
            &b.t.proposed_protocol_version_opt
        });

        test_path(&genesis, &fixture, "t/validation_callback", |b| {
            &b.t.validation_callback
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
        let et_deserialized = block_fixture.external_transitionv1().unwrap();
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
