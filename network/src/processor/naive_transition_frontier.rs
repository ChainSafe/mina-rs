// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! A naive implementation of the [TransitionFrontier]
//!

use super::*;
use mina_consensus::common::*;
use mina_rs_base::{types::*, verifiable::Verifiable};
use proof_systems::mina_signer::{self, NetworkId};
use tokio::sync::mpsc;

/// Struct that represents a naive implementation of the [TransitionFrontier]
#[derive(Debug, Clone, Default)]
pub struct NaiveTransitionFrontier {
    block_requester: Option<mpsc::Sender<QueryBlockRequest>>,
    best_chain: ProtocolStateChain,
}

impl NaiveTransitionFrontier {
    /// TODO: Doc
    pub fn new() -> Self {
        Default::default()
    }

    /// TODO: Doc
    pub fn get_best_chain(&self) -> &ProtocolStateChain {
        &self.best_chain
    }
}

#[async_trait(?Send)]
impl TransitionFrontier for NaiveTransitionFrontier {
    type Block = ExternalTransition;

    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>) {
        self.block_requester = Some(sender);
    }

    async fn add_block(&mut self, block: Self::Block) -> anyhow::Result<()> {
        let mut ctx = mina_signer::create_legacy::<SignedCommandPayload>(NetworkId::MAINNET);
        anyhow::ensure!(block.verify(&mut ctx), "block verification failure");
        if self.best_chain.length() < 1 {
            self.best_chain.push(block.protocol_state)?;
        } else {
            let candidate_chains = vec![ProtocolStateChain(vec![block.protocol_state])];
            // TODO: Avoid doing clone here by refining chain selection API(s)
            let best_chain = self.best_chain.select_secure_chain(&candidate_chains)?;
            self.best_chain = best_chain.clone();
        }
        Ok(())
    }
}
