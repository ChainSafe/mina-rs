// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};

use crate::{
    delta_transition_chain_proof::DeltaTransitionChainProofJson,
    protocol_state::ProtocolStateJson,
    protocol_state_proof::ProtocolStateProofBase64Json,
    protocol_version::ProtocolVersionJson,
    staged_ledger_diff::StagedLedgerDiffJson,
    v1::{
        DeltaTransitionChainProof, ProtocolStateProofV1, ProtocolStateV1, ProtocolVersionV1,
        StagedLedgerDiffV1,
    },
};
use versioned::*;

/// This structure represents a mina block received from an external block producer
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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
pub type ExternalTransitionV1 = Versioned<ExternalTransition, 1>;

/// This structure represents a mina block received from an external block producer
/// that is convertible from / to the mina specific json representation
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, AutoFrom)]
#[auto_from(ExternalTransition)]
pub struct ExternalTransitionJson {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: ProtocolStateJson,
    /// Proof that the protocol state and entire history of the chain is valid
    pub protocol_state_proof: ProtocolStateProofBase64Json,
    /// Diff of the proposed next state of the blockchain
    pub staged_ledger_diff: StagedLedgerDiffJson,
    /// Proof that the block was produced within the allotted slot time
    pub delta_transition_chain_proof: DeltaTransitionChainProofJson,
    /// Current protocol version
    #[serde(skip)]
    pub current_protocol_version: ProtocolVersionJson,
    /// Proposed protocol version
    #[serde(skip)]
    pub proposed_protocol_version_opt: Option<ProtocolVersionJson>,
    /// Callback used for validating external transition received over the network.
    /// This is not actually send over the network but requires a unit type to meet the
    /// serialization requirements
    #[serde(skip)]
    pub validation_callback: (),
}
