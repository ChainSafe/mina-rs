// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::types::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
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
    /// ?
    pub validation_callback: (),
}
