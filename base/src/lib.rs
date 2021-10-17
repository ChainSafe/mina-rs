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

pub mod blockchain_state;
pub mod consensus_state;
pub mod epoch_data;
pub mod global_slot;
pub mod numbers;
pub mod protocol_state;
pub mod protocol_version;
pub mod token_id;
