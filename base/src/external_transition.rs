// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Serialize, Deserialize};
use serde_versions_derive::version;

use crate::protocol_state::ProtocolState;
use crate::numbers::ProtocolVersion;
use crate::proof::Proof;
use mina_crypto::hash::StateHash;

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct ExternalTransition {
	protocol_state: ProtocolState,
	protocol_state_proof: Proof,
	staged_ledger_diff: u8,
	delta_transition_chain_proof: (StateHash, Vec<StateHash>),
	current_protocol_version: ProtocolVersion,
	proposed_protocol_version_opt: Option<ProtocolVersion>,
}
