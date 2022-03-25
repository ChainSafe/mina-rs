// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! This module defines functions that generate domain prefix from merkle tree node depth

/// Builds a hash prefix for a node at the given depth in a Merkle tree.
/// Note that this prefix does not have mina specific paddings.
pub fn make_prefix_merkle_tree(i: u32) -> String {
    format!("CodaMklTree{:03}", i)
}

/// Builds a hash prefix for a node at the given depth in a coinbase Merkle tree.
/// /// Note that this prefix does not have mina specific paddings.
pub fn make_prefix_coinbase_merkle_tree(i: u32) -> String {
    format!("CodaCbMklTree{:03}", i)
}
