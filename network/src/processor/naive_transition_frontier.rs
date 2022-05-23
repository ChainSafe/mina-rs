// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! A naive implementation of the [TransitionFrontier]
//!

use super::*;
use mina_consensus::common::*;
use mina_rs_base::types::*;
use tokio::sync::mpsc;

/// Struct that represents a naive implementation of the [TransitionFrontier]
pub struct NaiveTransitionFrontier {
    block_requester: mpsc::Sender<QueryBlockRequest>,
    best_chain: ProtocolStateChain,
}

impl NaiveTransitionFrontier {
    /// TODO: Doc
    pub fn get_best_chain_state(&self) -> Option<&ProtocolState> {
        self.best_chain.top()
    }
}

#[async_trait(?Send)]
impl TransitionFrontier for NaiveTransitionFrontier {
    type Block = ExternalTransition;

    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>) {
        self.block_requester = sender;
    }

    async fn add_block(&mut self, block: Self::Block) -> anyhow::Result<()> {
        if self.best_chain.length() < 1 {
            self.best_chain.push(block.protocol_state)?;
        } else {
            let candidate_chains = vec![ProtocolStateChain(vec![block.protocol_state])];
            // TODO: Avoiding doing clone here by refining chain selection API(s)
            let best_chain = self.best_chain.select_secure_chain(&candidate_chains)?;
            self.best_chain = best_chain.clone();
        }
        Ok(())
    }
}
