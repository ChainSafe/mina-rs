// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]
#![deny(missing_docs)]

//! mina-merkle crate provides traits and data structure implementations for
//! in-memory, persistent, maskable and masking mina merkle tree

mod proof;
pub use proof::*;
mod tree;
pub use tree::*;
mod tree_impl;
pub use tree_impl::*;
mod maskable;
pub use maskable::*;
mod masking;
pub use masking::*;
mod merger;
pub use merger::*;
mod merger_poseidon;
pub use merger_poseidon::*;
mod hasher;
pub use hasher::*;
mod hasher_poseidon;
pub use hasher_poseidon::*;
mod metadata;
pub use metadata::*;
pub mod prefixes;

use proof_systems::*;

/// Re-exports external types that macro implementations depend on,
/// so that the crate that uses the macros do not need to depend on
/// these external types
pub mod macros {
    pub use lockfree_object_pool;
    pub use once_cell;
    pub use proof_systems::*;
}
