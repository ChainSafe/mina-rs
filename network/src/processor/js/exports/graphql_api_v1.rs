// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Utilities for graphQL backend v1 at <https://graphql.minaexplorer.com/>
//!

use mina_serialization_types::json::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/graphql_v1_utils.js")]
extern "C" {
    pub async fn fetch_block_json_str(height: usize, state_hash: &str) -> JsValue;
}

/// TODO: Doc
pub async fn fetch_block(
    height: usize,
    state_hash: &str,
) -> Result<ExternalTransitionJson, serde_json::Error> {
    let json_str = fetch_block_json_str(height, state_hash)
        .await
        .as_string()
        .unwrap_or_default();
    serde_json::from_str(&json_str)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "browser")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    /// This currently fails with cors errors
    #[cfg(feature = "browser")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_fetch_block_json_str() {
        use super::*;

        let block_json_str = fetch_block_json_str(
            25718,
            "3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn",
        )
        .await
        .as_string()
        .unwrap();
        assert!(!block_json_str.is_empty());
    }

    /// This currently fails with cors errors
    #[cfg(feature = "browser")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_fetch_block() {
        use super::*;

        let block = fetch_block(
            25718,
            "3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn",
        )
        .await
        .unwrap();
        assert_eq!(
            block
                .protocol_state
                .body
                .genesis_state_hash
                .to_base58_string()
                .unwrap()
                .as_str(),
            "3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ"
        );
    }
}
