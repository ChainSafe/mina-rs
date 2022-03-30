// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! This module defines functions that generate domain prefix from merkle tree node height

/// Builds a hash prefix for a node at the given height in a Merkle tree (leaf nodes not counted, or counted as -1).
/// Note that this prefix does not have mina specific paddings.
pub fn make_prefix_merkle_tree(height: u32) -> String {
    format!("CodaMklTree{:03}", height)
}

/// Builds a hash prefix for a node at the given height in a coinbase Merkle tree (leaf nodes not counted, or counted as -1).
/// Note that this prefix does not have mina specific paddings.
pub fn make_prefix_coinbase_merkle_tree(height: u32) -> String {
    format!("CodaCbMklTree{:03}", height)
}
