// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Implementation agnostic abstraction of networking
//! operations and the transition frontier for mina node(s).
//!
//! The goal of this module is to support both graphql backend and libp2p backend
//! to power both wasm node(s) that run in browser and cli node(s)
//! that run on x86-64 and arm-64 CPU(s). It also allows easily mocking
//! the networking layer to unittest [TransitionFrontier]
//!

mod processor_impl;

use tokio::sync::mpsc;

/// Request struct for querying a block
pub struct QueryBlockRequest {
    /// Height of parent block
    pub height: usize,
    /// State hash of parent block
    pub state_hash: String,
}

/// TODO: Doc
/// Should this below to its own crate?
pub trait TransitionFrontier {
    /// Type that represents a block
    type Block;

    /// TODO: Doc
    fn add_block(&mut self, block: Self::Block);

    /// TODO: Doc
    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>);
}

/// abstraction of networking operations for
/// non-consensus mina node(s).
pub trait NonConsensusNetworkingOps {
    /// Type that represents a block
    type Block;

    /// TODO: Doc
    fn set_block_responder(&mut self, sender: mpsc::Sender<Self::Block>);

    /// TODO: Doc
    fn query_block(&self, request: &QueryBlockRequest);
}

/// TODO: Doc
pub struct NetworkMessageProcessor<Block, TF, NCOps>
where
    TF: TransitionFrontier<Block = Block>,
    NCOps: NonConsensusNetworkingOps<Block = Block>,
{
    /// TODO: Doc
    transition_frontier: TF,
    /// TODO: Doc
    nonconsensus_ops: NCOps,
    /// TODO: Doc
    block_receiver: mpsc::Receiver<Block>,
    /// TODO: Doc
    query_parent_block_request_receiver: mpsc::Receiver<QueryBlockRequest>,
}
