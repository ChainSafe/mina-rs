// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Utilities for graphQL backend on berkeley net at <http://localhost:3085/graphql>
//!

use crate::processor::*;
use hashbrown::HashSet;

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
}

#[async_trait(?Send)]
impl NonConsensusNetworkingOps for NonConsensusGraphQLBerkeleyBackend {
    type Block = serde_json::Value; //ProtocolState;

    fn set_block_responder(&mut self, sender: mpsc::Sender<Self::Block>) {
        self.block_responder = Some(sender);
    }

    async fn query_block(&mut self, _request: &QueryBlockRequest) -> anyhow::Result<()> {
        // if let Some(block_responder) = &self.block_responder {
        //     let block_json = fetch_block(request.height, request.state_hash.as_str()).await?;
        //     block_responder.send(block_json).await?;
        // }
        // Ok(())
        todo!()
    }
}
