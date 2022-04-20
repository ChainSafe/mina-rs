// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! All human readable values (e.g base58 encoded hashes and addresses)
//! implement the Base58Checked encoding <https://en.bitcoin.it/wiki/Base58Check_encoding>
//!
//! This adds a unique prefix byte to each type of encoding so they cannot be confused
//! (e.g. a hash cannot be used as an address). It also adds checksum bytes to the end.
//!

// TODO: Move the entire version_bytes.rs under crypto crate here

/// base58 version check byte for ledger hash
pub const LEDGER_HASH: u8 = 0x05;