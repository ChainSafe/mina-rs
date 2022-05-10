// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This crate implements a number of the base types required to define the Mina protocol.
//! This includes various types of state, blocks, slots, keypairs, numbers etc.
//! All types are versioned where required.
//!

#![deny(warnings)]
#![deny(missing_docs)]

pub mod account;
pub mod blockchain_state;
pub mod consensus_state;
mod constants;
pub mod delta_transition_chain_proof;
pub mod epoch_data;
pub mod external_transition;
pub mod global_slot;
pub mod numbers;
pub mod protocol_state;
pub mod protocol_state_proof;
pub mod protocol_version;
mod serialization_type_conversions;
pub mod snark_work;
pub mod staged_ledger_diff;
pub mod user_commands;
pub mod verifiable;
pub mod verification_key;

/// Import all crates from proof_systems
use proof_systems::*;

/// Re-export serialization type annotations
pub use mina_serialization_types::{BinProtSerializationType, JsonSerializationType};

/// Re-export all the public types under this module for convenience
pub mod types {
    pub use super::account::*;
    pub use super::blockchain_state::*;
    pub use super::consensus_state::*;
    pub use super::delta_transition_chain_proof::*;
    pub use super::epoch_data::*;
    pub use super::external_transition::*;
    pub use super::global_slot::*;
    pub use super::numbers::*;
    pub use super::protocol_state::*;
    pub use super::protocol_state_proof::*;
    pub use super::protocol_version::*;
    pub use super::snark_work::*;
    pub use super::staged_ledger_diff::*;
    pub use super::user_commands::*;
    pub use super::verification_key::*;
}
