// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use mina_crypto::prelude::*;
use serde::{Deserialize, Serialize};
use wire_type_2::WireType;

use crate::types::*;
use crate::network_types;

/// This structure represents a mina block
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// Callback used for validating external transition received over the network.
    pub validation_callback: (),
}

impl From<network_types::ExternalTransitionV1> for ExternalTransition {
    fn from(t: network_types::ExternalTransitionV1) -> Self {
        let t = t.0.inner();
        Self {
            protocol_state: t.protocol_state,
            protocol_state_proof: t.protocol_state_proof,
            staged_ledger_diff: t.staged_ledger_diff,
            delta_transition_chain_proof: t.delta_transition_chain_proof,
            current_protocol_version: t.current_protocol_version,
            proposed_protocol_version_opt: t.proposed_protocol_version_opt,
            validation_callback: t.validation_callback,
        }
    }
}

impl Into<network_types::ExternalTransitionV1> for ExternalTransition {
    fn into(self) -> network_types::ExternalTransitionV1 {
        unimplemented!()
    }
}

impl<'a> WireType<'a> for ExternalTransition {
    type WireType = network_types::ExternalTransitionV1;
}

impl BinProtEncodable for ExternalTransition {
    const PREALLOCATE_BUFFER_BYTES: usize = 13 * 1024;
}
