// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! js bindings and wasm implementation of graphql API(s)
//!

pub mod exports;

use self::exports::graphql_api_v1::fetch_block;
use crate::processor::*;
use async_trait::async_trait;
use mina_serialization_types::json::ExternalTransitionJson;

/// TODO: Doc
pub struct NonConsensusGraphQLV1Backend {
    sender: mpsc::Sender<ExternalTransitionJson>,
}

impl NonConsensusGraphQLV1Backend {
    /// TODO: Doc
    pub async fn poll_loop() {
        fetch_block(1, "").await.unwrap();
    }
}

#[async_trait(?Send)]
impl NonConsensusNetworkingOps for NonConsensusGraphQLV1Backend {
    type Block = ExternalTransitionJson;

    fn set_block_responder(&mut self, sender: mpsc::Sender<Self::Block>) {
        self.sender = sender;
    }

    async fn query_block(&mut self, request: &QueryBlockRequest) -> anyhow::Result<()> {
        let block_json = fetch_block(request.height, request.state_hash.as_str()).await?;
        self.sender.send(block_json).await?;
        Ok(())
    }
}
