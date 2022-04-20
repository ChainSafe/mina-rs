// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use crate::{types::*, SerializableTypeAnnotation};
use mina_serialization_types::external_transition::ExternalTransitionV1;

/// This structure represents a mina block
#[derive(Clone, Debug, PartialEq)]
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
}

impl SerializableTypeAnnotation for ExternalTransition {
    type BinProtType = ExternalTransitionV1;
    // TODO: Use actual ExternalTransitionV1Json when it's implemented
    type JsonType = ExternalTransitionV1;
}
