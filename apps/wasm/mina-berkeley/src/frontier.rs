// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use lazy_static::lazy_static;
use mina_network::processor::{js::graphql_api_berkeley::*, *};
use mina_rs_base::types::*;

type NaiveTransitionFrontierBerkeley =
    NaiveTransitionFrontier<ProtocolState, MerkleProofBerkeleyNet>;

type ProcessorTypeBerkeley = NetworkMessageProcessor<
    <NonConsensusGraphQLBerkeleyBackend as NonConsensusNetworkingOps>::Block,
    <NaiveTransitionFrontierBerkeley as TransitionFrontier>::Block,
    NaiveTransitionFrontierBerkeley,
    NonConsensusGraphQLBerkeleyBackend,
>;

lazy_static! {
    pub static ref PROCESSOR_BERKELEY: ProcessorTypeBerkeley = new_processor_berkeley();
}

fn new_processor_berkeley() -> ProcessorTypeBerkeley {
    let backend = NonConsensusGraphQLBerkeleyBackend::new();
    let frontier = NaiveTransitionFrontier::new();
    ProcessorTypeBerkeley::new(frontier, backend)
}
