// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use crate::{types::*, *, verifiable::Verifiable};
use proof_systems::mina_signer::Signer;

use mina_serialization_types::{json::ExternalTransitionJson, v1::ExternalTransitionV1};

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

impl BinProtSerializationType for ExternalTransition {
    type T = ExternalTransitionV1;
}

impl JsonSerializationType for ExternalTransition {
    type T = ExternalTransitionJson;
}

// impl<CTX> Verifiable<CTX> for ExternalTransition
// where
//     CTX: Signer<SignedCommandPayload>,
// {
//     fn verify(&self, _: &mut CTX) -> bool {
//         self.staged_ledger_diff.
//     }
// }