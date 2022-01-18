// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]
#![deny(missing_docs)]

//! mina-merkle crate provides traits and data structure implementations for
//! in-memory, persistent, maskable and masking mina merkle tree

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
mod hasher;
pub use hasher::*;
mod metadata;
pub use metadata::*;
