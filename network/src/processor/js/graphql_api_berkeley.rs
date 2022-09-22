// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Utilities for graphQL backend on berkeley net at <http://localhost:3085/graphql>
//!

use crate::processor::*;
use hashbrown::HashSet;
use js_sys::*;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/graphql_berkeley_utils.js")]
extern "C" {
    /// Fetch best chain json string
    #[wasm_bindgen(catch)]
    pub async fn fetch_best_chain_json_str(
        endpoint: &str,
        tracking_accounts: Vec<JsString>,
    ) -> Result<JsValue, JsValue>;
}

/// Fetch best chain json
pub async fn fetch_best_chain_json(
    endpoint: &str,
    tracking_accounts: Vec<String>,
) -> anyhow::Result<serde_json::Value> {
    let json = fetch_best_chain_json_str(
        endpoint,
        tracking_accounts.into_iter().map(|i| i.into()).collect(),
    )
    .await
    .map_err(|err| anyhow::Error::msg(err.as_string().unwrap_or_default()))?;
    Ok(serde_json::from_str(
        json.as_string().unwrap_or_default().as_str(),
    )?)
}

/// TODO: Doc
#[derive(Debug, Clone, Default)]
pub struct NonConsensusGraphQLBerkeleyBackend {
    block_responder: Option<mpsc::Sender<serde_json::Value>>,
    api_list: HashSet<String>,
    tracking_accounts: HashSet<String>,
}

impl NonConsensusGraphQLBerkeleyBackend {
    /// Creates a new instance
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets API list
    pub fn set_api_list(&mut self, api_urls: impl Iterator<Item = String>) {
        self.api_list = HashSet::from_iter(api_urls)
    }

    /// Sets tracking account list
    pub fn set_tracking_accounts(&mut self, account: impl Iterator<Item = String>) {
        self.tracking_accounts = HashSet::from_iter(account)
    }

    /// Poll latest blocks once
    pub async fn poll_latest_once(&self) -> anyhow::Result<()> {
        if let Some(block_responder) = &self.block_responder {
            let tracking_accounts: Vec<String> = self.tracking_accounts.iter().cloned().collect();
            for endpoint in &self.api_list {
                let block_json =
                    fetch_best_chain_json(endpoint.as_str(), tracking_accounts.clone()).await?;
                block_responder.send(block_json).await?;
            }
        }
        Ok(())
    }
}

#[async_trait(?Send)]
impl NonConsensusNetworkingOps for NonConsensusGraphQLBerkeleyBackend {
    type Block = serde_json::Value;

    fn set_block_responder(&mut self, sender: mpsc::Sender<Self::Block>) {
        self.block_responder = Some(sender);
    }

    async fn query_block(&mut self, _request: &QueryBlockRequest) -> anyhow::Result<()> {
        if let Some(block_responder) = &self.block_responder {
            let tracking_accounts: Vec<String> = self.tracking_accounts.iter().cloned().collect();
            for endpoint in &self.api_list {
                let block_json =
                    fetch_best_chain_json(endpoint.as_str(), tracking_accounts.clone()).await?;
                block_responder.send(block_json).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "browser")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    /// This test assumes a local mina node is running
    #[cfg(not(feature = "browser"))]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_fetch_best_chain_json() {
        use super::*;

        // NOTE: Disabled for CI
        if is_ci() {
            return;
        }

        let json = fetch_best_chain_json(
            "http://localhost:3085/graphql",
            vec![
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".into(),
                "B62qknCv9QdyAvt4Te58oo3nrZTacpEcjpJg1MV61r94h5rDPDUyPP8".into(),
            ],
        )
        .await
        .unwrap();
        assert!(json["bestChain"][0]["protocolState"].as_object().is_some());
        ProtocolStateWithSparseMerkleLedger::try_from(&json).unwrap();
    }

    /// This test assumes a local mina node is running
    #[cfg(not(feature = "browser"))]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn e2e_test_transition_frontier_berkeley() {
        use super::*;
        use mina_rs_base::types::*;

        // NOTE: Disabled for CI
        if is_ci() {
            return;
        }

        type NaiveTransitionFrontierBerkeleyNet =
            NaiveTransitionFrontier<ProtocolState, MerkleProofBerkeleyNet>;

        type ProcessorTypeBerkeleyNet = NetworkMessageProcessor<
            <NonConsensusGraphQLBerkeleyBackend as NonConsensusNetworkingOps>::Block,
            <NaiveTransitionFrontierBerkeleyNet as TransitionFrontier>::Block,
            NaiveTransitionFrontierBerkeleyNet,
            NonConsensusGraphQLBerkeleyBackend,
        >;

        let processor = {
            let backend = NonConsensusGraphQLBerkeleyBackend::new();
            let frontier = NaiveTransitionFrontier::new();
            ProcessorTypeBerkeleyNet::new(frontier, backend)
        };
        {
            let mut backend = processor.nonconsensus_ops_mut().await;
            backend
                .api_list
                .insert("http://localhost:3085/graphql".into());
            backend
                .tracking_accounts
                .insert("B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".into());
            backend
                .tracking_accounts
                .insert("B62qknCv9QdyAvt4Te58oo3nrZTacpEcjpJg1MV61r94h5rDPDUyPP8".into());
        }
        {
            let backend = processor.nonconsensus_ops().await;
            assert!(backend.poll_latest_once().await.is_ok());
        }
        // `processor.run()` needs to be executed to start the event loop
        // to actually update the transition frontier
    }

    fn is_ci() -> bool {
        match std::env::var("CI") {
            Ok(ci) => !ci.is_empty(),
            _ => false,
        }
    }
}
