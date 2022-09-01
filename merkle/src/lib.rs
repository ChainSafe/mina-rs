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
mod path;
pub use path::*;
pub mod prefixes;

use proof_systems::*;
