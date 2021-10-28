// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::types::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ExternalTransition {
    protocol_state: ProtocolState,
    protocol_state_proof: ProtocolStateProof,
    staged_ledger_diff: StagedLedgerDiff,
    delta_transition_chain_proof: DeltaTransitionChainProof,
    current_protocol_version: ProtocolVersion,
    proposed_protocol_version_opt: Option<ProtocolVersion>,
    validation_callback: (),
}
