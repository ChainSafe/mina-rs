// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Implementation agnostic abstraction of networking
//! operations and the transition frontier for mina node(s).
//!
//! The goal of this module is to provide extensible traits to
//! support graphql and libp2p backend, etc. to power
//! both wasm node(s) that run in browser and cli node(s)
//! that run on x86-64 and arm-64 CPU(s). It also makes it easy to mock
//! the networking layer to unittest the [TransitionFrontier]
//!

mod processor_impl;

use async_trait::async_trait;
use tokio::sync::mpsc;

/// Request struct for querying a block
pub struct QueryBlockRequest {
    /// Height of the block
    pub height: usize,
    /// State hash of the block
    pub state_hash: String,
}

/// This struct handles the blocks that are received from the
/// network and has the capability of interacting asynchronously
/// with the networking layer
///
/// TODO: Should this below to its own crate?
#[async_trait]
pub trait TransitionFrontier {
    /// Type that represents a block
    type Block;

    /// Adds a block that is received from networking layer.
    /// The block could be either pushed by other peers or
    /// the response of [QueryBlockRequest].
    async fn add_block(&mut self, block: Self::Block);

    /// Sets the block requester for querying a block, e.g. parent of certain block
    /// The responses will be recieved in a asynchronous way by the `add_block` API
    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>);
}

/// abstraction of networking operations for
/// non-consensus mina node(s).
#[async_trait]
pub trait NonConsensusNetworkingOps {
    /// Type that represents a block
    type Block;

    /// Sets the block responder that sends the blocks to the [TransitionFrontier]
    ///
    /// Note that it only assumes that new blocks are being pushed
    /// from the network, to support polling with the graphql API(s),
    /// there should be a separate long running function in the [NonConsensusNetworkingOps] impl
    /// that is invoked or scheduled separately
    fn set_block_responder(&mut self, sender: mpsc::Sender<Self::Block>);

    /// Queries a block with its height and state hash
    async fn query_block(&mut self, request: &QueryBlockRequest);
}

/// This struct processes all the interactions and data exchanges
/// between the [NonConsensusNetworkingOps] and the [TransitionFrontier]
pub struct NetworkMessageProcessor<Block, TF, NCOps>
where
    TF: TransitionFrontier<Block = Block>,
    NCOps: NonConsensusNetworkingOps<Block = Block>,
{
    /// The [TransitionFrontier] instance
    transition_frontier: TF,
    /// The [NonConsensusNetworkingOps] instance
    nonconsensus_ops: NCOps,
    /// Block receiver
    block_receiver: mpsc::Receiver<Block>,
    /// [QueryBlockRequest] receiver
    query_block_request_receiver: mpsc::Receiver<QueryBlockRequest>,
}
