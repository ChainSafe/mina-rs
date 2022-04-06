// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Adds support for serializing and deserializing polyvars as Rust enums

mod caml_hash_variant;
pub use caml_hash_variant::{caml_hash_variant, VariantHash};
