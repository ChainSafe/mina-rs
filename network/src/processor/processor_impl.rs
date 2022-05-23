// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use tokio::sync::RwLockReadGuard;

use super::*;
use std::marker::{Send, Sync};

impl<'a, NetworkBlock, FrontierBlock, TF, NCOps>
    NetworkMessageProcessor<NetworkBlock, FrontierBlock, TF, NCOps>
where
    TF: TransitionFrontier<Block = FrontierBlock>,
    NCOps: NonConsensusNetworkingOps<Block = NetworkBlock>,
    NetworkBlock: Send + Sync,
    FrontierBlock: From<NetworkBlock>,
{
    /// Creates a new [NetworkMessageProcessor] with the given [TransitionFrontier] and
    /// [NonConsensusNetworkingOps], initializing all the message channels for the communication
    pub fn new(mut transition_frontier: TF, mut nonconsensus_ops: NCOps) -> Self {
        // TODO: make buffer size configurable, use rendezvous channel for now
        let (sender, block_receiver) = mpsc::channel(1);
        nonconsensus_ops.set_block_responder(sender);

        // TODO: make buffer size configurable, use rendezvous channel for now
        let (sender, query_block_request_receiver) = mpsc::channel::<QueryBlockRequest>(1);
        transition_frontier.set_block_requester(sender);

        Self {
            transition_frontier: RwLock::new(transition_frontier),
            nonconsensus_ops: RwLock::new(nonconsensus_ops),
            block_receiver: RwLock::new(block_receiver),
            query_block_request_receiver: RwLock::new(query_block_request_receiver),
        }
    }

    /// Gets the [TransitionFrontier] instance
    pub async fn transition_frontier(&'a self) -> RwLockReadGuard<'a, TF> {
        self.transition_frontier.read().await
    }

    /// Gets the [NonConsensusNetworkingOps] instance
    pub async fn nonconsensus_ops(&'a self) -> RwLockReadGuard<'a, NCOps> {
        self.nonconsensus_ops.read().await
    }

    /// Schedules event loops of all types of communications between [TransitionFrontier] and
    /// [NonConsensusNetworkingOps].
    pub async fn run(&self) {
        // let block_receiver = &mut self.block_receiver;
        // let transition_frontier = &mut self.transition_frontier;
        // let query_block_request_receiver = &mut self.query_block_request_receiver;
        // let nonconsensus_ops = &mut self.nonconsensus_ops;
        _ = tokio::join!(self.run_recv_block_loop(), self.run_query_block_loop());
    }

    /// Schedules the event loop of sending blocks that are received from the network
    /// to the [TransitionFrontier]
    async fn run_recv_block_loop(&self) {
        let mut block_receiver = self.block_receiver.write().await;
        while let Some(block) = block_receiver.recv().await {
            let mut transition_frontier = self.transition_frontier.write().await;
            _ = transition_frontier.add_block(block.into()).await;
        }
    }

    /// Schedules the event loop of sending query-block requests
    /// to the [NonConsensusNetworkingOps]
    async fn run_query_block_loop(&self) {
        let mut query_block_request_receiver = self.query_block_request_receiver.write().await;
        while let Some(request) = query_block_request_receiver.recv().await {
            let mut nonconsensus_ops = self.nonconsensus_ops.write().await;
            _ = nonconsensus_ops.query_block(&request).await;
        }
    }
}
