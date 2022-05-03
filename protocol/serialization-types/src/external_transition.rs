// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use serde::{Deserialize, Serialize};

use crate::{
    delta_transition_chain_proof::DeltaTransitionChainProofJson,
    protocol_state::ProtocolStateJson,
    protocol_state_proof::ProtocolStateProofJson,
    protocol_version::ProtocolVersionJson,
    staged_ledger_diff::StagedLedgerDiffJson,
    v1::{
        DeltaTransitionChainProof, ProtocolStateProofV1, ProtocolStateV1, ProtocolVersionV1,
        StagedLedgerDiffV1,
    },
};
use versioned::Versioned;

/// This structure represents a mina block received from an external block producer
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalTransitionV1(pub Versioned<ExternalTransition, 1>);

impl bin_prot::encodable::BinProtEncodable for ExternalTransitionV1 {
    const PREALLOCATE_BUFFER_BYTES: usize = 1800 * 1024;
}

/// This structure represents a mina block received from an external block producer
/// that is convertible from / to the mina specific json representation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalTransitionJson {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: ProtocolStateJson,
    /// Proof that the protocol state and entire history of the chain is valid
    pub protocol_state_proof: ProtocolStateProofJson,
    /// Diff of the proposed next state of the blockchain
    pub staged_ledger_diff: StagedLedgerDiffJson,
    /// Proof that the block was produced within the allotted slot time
    pub delta_transition_chain_proof: DeltaTransitionChainProofJson,
    /// Current protocol version
    pub current_protocol_version: ProtocolVersionJson,
    /// Proposed protocol version
    pub proposed_protocol_version_opt: Option<ProtocolVersionJson>,
    /// Callback used for validating external transition received over the network.
    /// This is not actually send over the network but requires a unit type to meet the
    /// serialization requirements
    pub validation_callback: (),
}

impl From<ExternalTransitionJson> for ExternalTransitionV1 {
    fn from(t: ExternalTransitionJson) -> Self {
        let t: ExternalTransition = t.into();
        Self(t.into())
    }
}

impl From<ExternalTransitionV1> for ExternalTransitionJson {
    fn from(t: ExternalTransitionV1) -> Self {
        t.0.t.into()
    }
}

impl From<ExternalTransitionJson> for ExternalTransition {
    fn from(t: ExternalTransitionJson) -> Self {
        Self {
            protocol_state: t.protocol_state.into(),
            protocol_state_proof: t.protocol_state_proof.into(),
            staged_ledger_diff: t.staged_ledger_diff.into(),
            delta_transition_chain_proof: t.delta_transition_chain_proof.into(),
            current_protocol_version: t.current_protocol_version.into(),
            proposed_protocol_version_opt: t.proposed_protocol_version_opt.map(|i| i.into()),
            validation_callback: t.validation_callback,
        }
    }
}

impl From<ExternalTransition> for ExternalTransitionJson {
    fn from(t: ExternalTransition) -> Self {
        Self {
            protocol_state: t.protocol_state.into(),
            protocol_state_proof: t.protocol_state_proof.into(),
            staged_ledger_diff: t.staged_ledger_diff.into(),
            delta_transition_chain_proof: t.delta_transition_chain_proof.into(),
            current_protocol_version: t.current_protocol_version.into(),
            proposed_protocol_version_opt: t.proposed_protocol_version_opt.map(|i| i.into()),
            validation_callback: t.validation_callback,
        }
    }
}
