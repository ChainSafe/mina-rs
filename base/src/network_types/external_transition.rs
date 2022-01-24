// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use mina_crypto::prelude::BinProtEncodable;
use serde::{Deserialize, Serialize};

use crate::network_types::v1::{ProtocolStateV1, StagedLedgerDiffV1}; // TODO: aim to remove this dep
use crate::types::{DeltaTransitionChainProof, ProtocolStateProof, ProtocolVersion};
use versioned::Versioned;

/// This structure represents a mina block received from an external block producer
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalTransition {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: ProtocolStateV1,
    /// Proof that the protocol state and entire history of the chain is valid
    pub protocol_state_proof: ProtocolStateProof,
    /// Diff of the proposed next state of the blockchain
    pub staged_ledger_diff: StagedLedgerDiffV1,
    /// Proof that the block was produced within the allotted slot time
    pub delta_transition_chain_proof: DeltaTransitionChainProof,
    /// Current protocol version
    pub current_protocol_version: ProtocolVersion,
    /// Proposed protocol version
    pub proposed_protocol_version_opt: Option<ProtocolVersion>,
    /// Callback used for validating external transition received over the network.
    /// This is not actually send over the network but requires a unit type to meet the
    /// serialization requirements
    pub validation_callback: (),
}

/// Versioned structure to use externally
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalTransitionV1(pub Versioned<ExternalTransition, 1>);

impl BinProtEncodable for ExternalTransitionV1 {
    const PREALLOCATE_BUFFER_BYTES: usize = 13 * 1024;
}
