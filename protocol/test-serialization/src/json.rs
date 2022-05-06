// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use crate::*;
    use mina_rs_base::{types::*, *};
    use std::str::FromStr;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn consensus_state_json_serde_roundtrip() {
        json_serde_roundtrip!(ConsensusState, "protocol_state/body/consensus_state");
    }

    #[test]
    #[wasm_bindgen_test]
    fn constants_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolConstants, "protocol_state/body/constants");
    }

    #[test]
    #[wasm_bindgen_test]
    fn blockchain_state_json_serde_roundtrip() {
        json_serde_roundtrip!(BlockchainState, "protocol_state/body/blockchain_state");
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_body_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolStateBody, "protocol_state/body");
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolState, "protocol_state");
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolStateProof, "protocol_state_proof");
    }

    #[test]
    #[wasm_bindgen_test]
    fn delta_transition_chain_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(DeltaTransitionChainProof, "delta_transition_chain_proof");
    }

    #[test]
    // #[wasm_bindgen_test]
    #[should_panic] // Not fully implemented yet
    fn staged_ledger_diff_json_serde_roundtrip() {
        json_serde_roundtrip!(StagedLedgerDiff, "staged_ledger_diff");
    }

    #[test]
    // #[wasm_bindgen_test]
    #[should_panic] // Not fully implemented yet
    fn block_json_serde_roundtrip() {
        json_serde_roundtrip!(ExternalTransition, "");
    }

    #[macro_export]
    macro_rules! json_serde_roundtrip {
        ($ty: ty, $path: literal) => {
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
                    let cs = {
                        let json_string = serde_json::to_string_pretty(json)?;
                        <$ty>::try_from_json(json_string.as_str())?
                    };
                    let json_string_from_cs = cs.try_into_json()?;
                    let json_from_cs: serde_json::Value =
                        serde_json::from_str(&json_string_from_cs)?;
                    assert_eq!(json, &json_from_cs);
                }
                Ok::<_, anyhow::Error>(())
            })()
            .unwrap();
        };
    }
}
