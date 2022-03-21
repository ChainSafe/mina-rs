// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Adds support for serializing and deserializing polyvars as Rust enums

use serde::{Deserialize, Serialize};

mod caml_hash_variant;
pub use caml_hash_variant::{caml_hash_variant, VariantHash};

/// A polymorphic variant as it should be serialized, the variant hash followed by the body.
/// The default implementations of Serialize and Deserialize work for this.
#[derive(Serialize, Deserialize)]
pub struct PolyVar<T> {
    /// The variant hash computed by using `caml_hash_variant` on its label
    hash: VariantHash,
    /// The body associated with the variant. Can be empty.
    body: T,
}
