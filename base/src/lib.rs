// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This crate implements a number of the base types required to define the Mina protocol.
//! This includes various types of state, blocks, slots, keypairs, numbers etc.
//! All types are versioned where required.
//!

// Need to supress this warning for the moment so we can use the #[verson(x)]
// attribute macro. It seems we are in an awkward inbetween phase where it can't be
// defined above a derive macro (or the following warning) and it can't be defined
// below or it will error. The error will be fixed in the future when derive becomes like a regular
// attribute macro and an order of operations well defined
#![allow(legacy_derive_helpers)]
#![deny(rustdoc::all)]

pub mod blockchain_state;
pub mod consensus_state;
pub mod delta_transition_chain_proof;
pub mod epoch_data;
pub mod external_transition;
pub mod global_slot;
pub mod numbers;
pub mod protocol_state;
pub mod protocol_state_proof;
pub mod protocol_version;
pub mod staged_ledger_diff;
pub mod token_id;
pub mod verification_key;

pub mod types {
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
    pub use super::staged_ledger_diff::*;
    pub use super::token_id::*;
    pub use super::verification_key::*;
}
