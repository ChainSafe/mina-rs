// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Contains definitions of common simple crypto primitives used in the Mina protocol
//! This currently includes keys and hashes along with their encodings
//!

#![deny(warnings)]

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
    pub use crate::hex::HexEncodable;
}

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
