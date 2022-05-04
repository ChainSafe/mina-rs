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

/// base58 version check byte for coinbase hash
pub const COINBASE_HASH: u8 = 0x0c;

/// base58 version check byte for epoch seed hash
pub const EPOCH_SEED: u8 = 0x0d;

/// base58 version check byte for state hash
pub const STATE_HASH: u8 = 0x10;

/// base58 version check byte for vrf output hash
pub const VRF_TRUNCATED_OUTPUT: u8 = 0x15;

/// base58 version check byte for aux hash
pub const STAGED_LEDGER_HASH_AUX_HASH: u8 = 0x0E;

/// base58 version check byte for pending coinbase aux hash
pub const STAGED_LEDGER_HASH_PENDING_COINBASE_AUX: u8 = 0x0F;

/// base58 version check byte for compressed curve point (public key)
pub const COMPRESSED_CURVE_POINT: u8 = 0xCB;
