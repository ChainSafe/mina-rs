// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use lazy_static::lazy_static;
use mina_network::processor::{js::graphql_api_v1::*, *};

type ProcessorType = NetworkMessageProcessor<
    <NonConsensusGraphQLV1Backend as NonConsensusNetworkingOps>::Block,
    <NaiveTransitionFrontier as TransitionFrontier>::Block,
    NaiveTransitionFrontier,
    NonConsensusGraphQLV1Backend,
>;

lazy_static! {
    pub static ref PROCESSOR: ProcessorType = new_processor();
}

fn new_processor() -> ProcessorType {
    let backend = NonConsensusGraphQLV1Backend::new();
    let frontier = NaiveTransitionFrontier::new();
    ProcessorType::new(frontier, backend)
}
