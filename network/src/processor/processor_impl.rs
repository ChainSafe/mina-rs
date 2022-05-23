// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use std::marker::{Send, Sync};

impl<NetworkBlock, FrontierBlock, TF, NCOps>
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
            transition_frontier,
            nonconsensus_ops,
            block_receiver,
            query_block_request_receiver,
        }
    }

    /// Schedules event loops of all types of communications between [TransitionFrontier] and
    /// [NonConsensusNetworkingOps].
    pub async fn run(&mut self) {
        let block_receiver = &mut self.block_receiver;
        let transition_frontier = &mut self.transition_frontier;
        let query_block_request_receiver = &mut self.query_block_request_receiver;
        let nonconsensus_ops = &mut self.nonconsensus_ops;
        _ = tokio::join!(
            Self::run_recv_block_loop(transition_frontier, block_receiver),
            Self::run_query_block_loop(nonconsensus_ops, query_block_request_receiver)
        );
    }

    /// Schedules the event loop of sending blocks that are received from the network
    /// to the [TransitionFrontier]
    async fn run_recv_block_loop(
        transition_frontier: &mut TF,
        block_receiver: &mut mpsc::Receiver<NetworkBlock>,
    ) {
        while let Some(block) = block_receiver.recv().await {
            _ = transition_frontier.add_block(block.into()).await;
        }
    }

    /// Schedules the event loop of sending query-block requests
    /// to the [NonConsensusNetworkingOps]
    async fn run_query_block_loop(
        nonconsensus_ops: &mut NCOps,
        query_block_request_receiver: &mut mpsc::Receiver<QueryBlockRequest>,
    ) {
        while let Some(request) = query_block_request_receiver.recv().await {
            _ = nonconsensus_ops.query_block(&request).await;
        }
    }
}
