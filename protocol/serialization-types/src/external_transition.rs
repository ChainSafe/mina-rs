// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use serde::{Deserialize, Serialize};

use crate::v1::{
    DeltaTransitionChainProof, ProtocolStateProofV1, ProtocolStateV1, ProtocolVersionV1,
    StagedLedgerDiffV1,
};
use versioned::Versioned;

/// This structure represents a mina block received from an external block producer
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalTransition {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: ProtocolStateV1,
    /// Proof that the protocol state and entire history of the chain is valid
    pub protocol_state_proof: ProtocolStateProofV1,
    /// Diff of the proposed next state of the blockchain
    pub staged_ledger_diff: StagedLedgerDiffV1,
    /// Proof that the block was produced within the allotted slot time
    pub delta_transition_chain_proof: DeltaTransitionChainProof,
    /// Current protocol version
    pub current_protocol_version: ProtocolVersionV1,
    /// Proposed protocol version
    pub proposed_protocol_version_opt: Option<ProtocolVersionV1>,
    /// Callback used for validating external transition received over the network.
    /// This is not actually send over the network but requires a unit type to meet the
    /// serialization requirements
    pub validation_callback: (),
}

/// Versioned structure to use externally
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalTransitionV1(pub Versioned<ExternalTransition, 1>);

impl mina_crypto::binprot::BinProtEncodable for ExternalTransitionV1 {
    const PREALLOCATE_BUFFER_BYTES: usize = 13 * 1024;
}
