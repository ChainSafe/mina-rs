// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use serde::{Deserialize, Serialize};

use crate::types::*;

/// This structure represents a mina block
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
/// This structure represents a mina block received from an external block producer
pub struct ExternalTransition {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: ProtocolState,
    /// Proof that the protocol state and entire history of the chain is valid
    pub protocol_state_proof: ProtocolStateProof,
    /// Diff of the proposed next state of the blockchain
    pub staged_ledger_diff: StagedLedgerDiff,
    /// Proof that the block was produced within the allotted slot time
    pub delta_transition_chain_proof: DeltaTransitionChainProof,
    /// Current protocol version
    pub current_protocol_version: ProtocolVersion,
    /// Proposed protocol version
    pub proposed_protocol_version_opt: Option<ProtocolVersion>,
    /// Callback used for validating external transition received over the network.
    pub validation_callback: (),
}

impl bin_prot::encodable::BinProtEncodable for ExternalTransition {
    const PREALLOCATE_BUFFER_BYTES: usize = 13 * 1024;
}
