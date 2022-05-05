// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use crate::*;
    use mina_rs_base::{types::*, JsonSerializationType};
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
    fn protocol_state_body_state_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolStateBody, "protocol_state/body");
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_state_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolState, "protocol_state");
    }

    #[macro_export]
    macro_rules! json_serde_roundtrip {
        ($ty: ty, $path: literal) => {
            (|| {
                for (_, mut json) in test_fixtures::JSON_TEST_BLOCKS.iter() {
                    for p in $path.split('/') {
                        json = &json[p];
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
