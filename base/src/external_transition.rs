// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina ExternalTransition

use mina_crypto::prelude::*;
use serde::{Deserialize, Serialize};
use wire_type_2::WireType;
use versioned::Versioned;

use crate::types::*;

/// This structure represents a mina block
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
/// This structure represents a mina block received from an external block producer
pub struct ExternalTransition {
    /// The blockchain state, including consensus and the ledger
    pub protocol_state: crate::types::ProtocolState,
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

use crate::network_types::v1::ExternalTransitionV1;

impl From<ExternalTransitionV1> for ExternalTransition {
    fn from(t: ExternalTransitionV1) -> Self {
        let t = t.0.inner();
        Self {
            protocol_state: t.protocol_state.into(),
            protocol_state_proof: t.protocol_state_proof,
            staged_ledger_diff: t.staged_ledger_diff.into(),
            delta_transition_chain_proof: t.delta_transition_chain_proof,
            current_protocol_version: t.current_protocol_version,
            proposed_protocol_version_opt: t.proposed_protocol_version_opt,
        }
    }
}

impl Into<ExternalTransitionV1> for ExternalTransition {
    fn into(self) -> ExternalTransitionV1 {
        ExternalTransitionV1(
            Versioned::new(crate::network_types::external_transition::ExternalTransition {
                protocol_state: self.protocol_state.into(),
                protocol_state_proof: self.protocol_state_proof,
                staged_ledger_diff: self.staged_ledger_diff.into(),
                delta_transition_chain_proof: self.delta_transition_chain_proof,
                current_protocol_version: self.current_protocol_version,
                proposed_protocol_version_opt: self.proposed_protocol_version_opt,
                validation_callback: (),                
            })
        )
    }
}

impl<'a> WireType<'a> for ExternalTransition {
    type WireType = ExternalTransitionV1;
}

impl BinProtEncodable for ExternalTransition {
    const PREALLOCATE_BUFFER_BYTES: usize = 13 * 1024;
}
