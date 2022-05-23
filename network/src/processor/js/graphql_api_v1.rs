// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Utilities for graphQL backend v1 at <https://graphql.minaexplorer.com/>
//!

use crate::processor::*;
use mina_serialization_types::json::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/graphql_v1_utils.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn fetch_block_json_str(height: usize, state_hash: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn query_latest_blocks_json_str(limit: usize) -> Result<JsValue, JsValue>;
}

/// Fetch [ExternalTransitionJson] with its height and state hash
pub async fn fetch_block(
    height: usize,
    state_hash: &str,
) -> anyhow::Result<ExternalTransitionJson> {
    let json_str = fetch_block_json_str(height, state_hash)
        .await
        .map_err(|err| anyhow::Error::msg(format!("{:?}", err)))?
        .as_string()
        .unwrap_or_default();
    Ok(serde_json::from_str(&json_str)?)
}

/// Query infomation of the latest blocks
pub async fn query_latest_blocks(limit: usize) -> anyhow::Result<Vec<BlockBasicInfo>> {
    let json_str = query_latest_blocks_json_str(limit)
        .await
        .map_err(|err| anyhow::Error::msg(format!("{:?}", err)))?
        .as_string()
        .unwrap_or_default();
    Ok(serde_json::from_str(&json_str)?)
}

/// Basic block information as query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBasicInfo {
    /// Block height
    #[serde(rename = "blockHeight")]
    pub block_height: usize,

    /// Block state hash
    #[serde(rename = "stateHash")]
    pub state_hash: String,
}

/// TODO: Doc
#[derive(Debug, Clone, Default)]
pub struct NonConsensusGraphQLV1Backend {
    block_responder: Option<mpsc::Sender<ExternalTransitionJson>>,
}

impl NonConsensusGraphQLV1Backend {
    /// TODO: Doc
    pub fn new() -> Self {
        Default::default()
    }

    /// TODO: Doc
    pub async fn poll_latest_once(&self) -> anyhow::Result<()> {
        if let Some(block_responder) = &self.block_responder {
            let blocks = query_latest_blocks(10).await?;
            for b in blocks {
                if let Ok(block) = fetch_block(b.block_height, b.state_hash.as_str()).await {
                    _ = block_responder.send(block).await;
                }
            }
        }
        Ok(())
    }
}

#[async_trait(?Send)]
impl NonConsensusNetworkingOps for NonConsensusGraphQLV1Backend {
    type Block = ExternalTransitionJson;

    fn set_block_responder(&mut self, sender: mpsc::Sender<Self::Block>) {
        self.block_responder = Some(sender);
    }

    async fn query_block(&mut self, request: &QueryBlockRequest) -> anyhow::Result<()> {
        if let Some(block_responder) = &self.block_responder {
            let block_json = fetch_block(request.height, request.state_hash.as_str()).await?;
            block_responder.send(block_json).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "browser")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    // This currently fails with cors errors in browser
    #[cfg(not(feature = "browser"))]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_fetch_block_json_str() {
        use super::*;

        let block_json_str = fetch_block_json_str(
            25718,
            "3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn",
        )
        .await
        .unwrap()
        .as_string()
        .unwrap();
        assert!(!block_json_str.is_empty());
    }

    // This currently fails with cors errors in browser
    #[cfg(not(feature = "browser"))]
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

    // This currently fails with cors errors in browser
    #[cfg(not(feature = "browser"))]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_query_latest_blocks() {
        use super::*;

        let limit = 10;
        let blocks = query_latest_blocks(limit).await.unwrap();
        assert_eq!(limit, blocks.len());
        for block in blocks {
            assert!(block.block_height > 0);
            assert!(!block.state_hash.is_empty());
        }
    }
}
