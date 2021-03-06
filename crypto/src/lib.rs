// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Contains definitions of common simple crypto primitives used in the Mina protocol
//! This currently includes keys and hashes along with their encodings
//!

#![deny(warnings)]

pub mod hash;
pub mod hex;
mod serialization_type_conversions;

pub mod prelude {
    pub use crate::hex::HexEncodable;
}
