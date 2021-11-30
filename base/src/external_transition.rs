// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::BinProtDeser;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::types::*;

/// This structure represents a mina block
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ExternalTransition {
    pub protocol_state: ProtocolState,
    pub protocol_state_proof: ProtocolStateProof,
    pub staged_ledger_diff: StagedLedgerDiff,
    pub delta_transition_chain_proof: DeltaTransitionChainProof,
    pub current_protocol_version: ProtocolVersion,
    pub proposed_protocol_version_opt: Option<ProtocolVersion>,
    pub validation_callback: (),
}

impl BinProtDeser for ExternalTransition {
    const PREALLOCATE_BUFFER_BYTES: usize = 13 * 1024;
}
