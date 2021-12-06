// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![allow(legacy_derive_helpers)]
#![deny(rustdoc::all)]

//!
//! Contains definitions of common simple crypto primitives used in the Mina protocol
//! This currently includes keys and hashes along with their encodings
//!

// Need to supress this warning for the moment so we can use the #[verson(x)]
// attribute macro. It seems we are in an awkward inbetween phase where it can't be
// defined above a derive macro (or the following warning) and it can't be defined
// below or it will error. The error will be fixed in the future when derive becomes like a regular
// attribute macro and an order of operations well defined

pub mod base58;
pub mod base64;
pub mod binprot;
pub mod hash;
pub mod hex;
pub mod signature;

pub mod prelude {
    pub use crate::base58::Base58Encodable;
    pub use crate::base64::Base64Encodable;
    pub use crate::binprot::BinProtEncodable;
    pub use crate::hash::*;
    pub use crate::hex::HexEncodable;
    pub use crate::signature::*;
}
