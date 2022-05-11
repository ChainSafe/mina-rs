// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use crate::*;
    use mina_rs_base::types::*;
    use mina_serialization_types::json::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn consensus_state_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ConsensusState,
            ConsensusStateJson,
            "protocol_state/body/consensus_state"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn constants_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolConstants,
            ProtocolConstantsJson,
            "protocol_state/body/constants"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn blockchain_state_json_serde_roundtrip() {
        json_serde_roundtrip!(
            BlockchainState,
            BlockchainStateJson,
            "protocol_state/body/blockchain_state"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_body_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolStateBody,
            ProtocolStateBodyJson,
            "protocol_state/body"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolState, ProtocolStateJson, "protocol_state");
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolStateProof,
            ProtocolStateProofBase64Json,
            "protocol_state_proof"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn delta_transition_chain_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(
            DeltaTransitionChainProof,
            DeltaTransitionChainProofJson,
            "delta_transition_chain_proof"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_statement_json_serde_roundtrip() {
        json_serde_roundtrip!(
            Statement,
            StatementJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/statement"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolStateProof,
            ProtocolStateProofJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_statement_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProofStatement,
            ProofStatementJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof/statement"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_statement_proof_state_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProofState,
            ProofStateJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof/statement/proof_state"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_statement_proof_state_deferred_values_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProofStateDeferredValues,
            ProofStateDeferredValuesJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof/statement/proof_state/deferred_values"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_statement_proof_state_deferred_values_b_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ShiftedValue,
            ShiftedValueMinaJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof/statement/proof_state/deferred_values/b"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_json_serde_roundtrip() {
        json_serde_roundtrip!(
            TransactionSnarkWork,
            TransactionSnarkWorkJson,
            "staged_ledger_diff/diff/0/completed_works/0"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn commands_json_serde_roundtrip() {
        json_serde_roundtrip!(
            UserCommandWithStatus,
            UserCommandWithStatusJson,
            "staged_ledger_diff/diff/0/commands/0"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn user_command_json_serde_roundtrip() {
        json_serde_roundtrip!(
            UserCommand,
            UserCommandMinaJson,
            "staged_ledger_diff/diff/0/commands/0/data"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn transaction_status_json_serde_roundtrip() {
        json_serde_roundtrip!(
            TransactionStatus,
            TransactionStatusMinaJson,
            "staged_ledger_diff/diff/0/commands/0/status"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn internal_command_balances_json_serde_roundtrip() {
        json_serde_roundtrip!(
            InternalCommandBalanceData,
            InternalCommandBalanceDataMinaJson,
            "staged_ledger_diff/diff/0/internal_command_balances/0"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn coinbase_json_serde_roundtrip() {
        json_serde_roundtrip!(
            CoinBase,
            CoinBaseMinaJson,
            "staged_ledger_diff/diff/0/coinbase"
        );
    }

    #[test]
    // #[wasm_bindgen_test]
    #[should_panic] // Not fully implemented yet
    fn staged_ledger_diff_json_serde_roundtrip() {
        json_serde_roundtrip!(StagedLedgerDiff, StagedLedgerDiffJson, "staged_ledger_diff");
    }

    #[test]
    // #[wasm_bindgen_test]
    #[should_panic] // Not fully implemented yet
    fn block_json_serde_roundtrip() {
        json_serde_roundtrip!(ExternalTransition, ExternalTransitionJson, "");
    }

    #[macro_export]
    macro_rules! json_serde_roundtrip {
        ($ty: ty, $ty_json: ty, $path: literal) => {
            (|| {
                'outer: for (_, mut json) in test_fixtures::JSON_TEST_BLOCKS.iter() {
                    if $path.len() > 0 {
                        for p in $path.split('/') {
                            json = match usize::from_str(p) {
                                Ok(index) => {
                                    if let Some(array) = json.as_array() {
                                        if index >= array.len() {
                                            continue 'outer;
                                        }
                                        &array[index]
                                    } else {
                                        panic!("Array expect");
                                    }
                                }
                                _ => &json[p],
                            };
                        }
                    }
                    let cs: $ty = {
                        let json_string = serde_json::to_string_pretty(json)?;
                        let json: $ty_json = serde_json::from_str(json_string.as_str())
                            .map_err(|err| anyhow::Error::msg(format!("{json_string}\n\n{err}")))?;
                        json.into()
                    };
                    let json_from_cs = {
                        let json: $ty_json = cs.into();
                        serde_json::to_value(&json)?
                    };
                    assert_eq!(json, &json_from_cs);
                }
                Ok::<_, anyhow::Error>(())
            })()
            .unwrap();
        };
    }
}
