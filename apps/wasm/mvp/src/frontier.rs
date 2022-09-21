// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use lazy_static::lazy_static;
use mina_network::processor::{js::graphql_api_v1::*, *};
use mina_rs_base::types::*;

type NaiveTransitionFrontierMainnet =
    NaiveTransitionFrontier<ProtocolStateLegacy, MerkleProofMainnet>;

type ProcessorTypeMainnet = NetworkMessageProcessor<
    <NonConsensusGraphQLV1Backend as NonConsensusNetworkingOps>::Block,
    <NaiveTransitionFrontierMainnet as TransitionFrontier>::Block,
    NaiveTransitionFrontierMainnet,
    NonConsensusGraphQLV1Backend,
>;

lazy_static! {
    pub static ref PROCESSOR_MAINNET: ProcessorTypeMainnet = new_processor_mainnet();
}

fn new_processor_mainnet() -> ProcessorTypeMainnet {
    let backend = NonConsensusGraphQLV1Backend::new();
    let frontier = NaiveTransitionFrontier::new();
    ProcessorTypeMainnet::new(frontier, backend)
}
