// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Traits and helpers for data structures that require validation

/// Types that implement Verifiable are able to self-check using intrinsic data (e.g. signatures)
/// and optionally some context that is required for peforming the verification
/// e.g. a singleton signature verifier or param required for snark verification
pub trait Verifiable<CTX> {
    /// Accepts self and a context and
    /// returns if the implementor is valid
    ///
    /// The context must be a mutable reference to satisfy the requirements of using a
    /// shared signature checking context which performs self mutation as an optimization
    fn verify(&self, ctx: &mut CTX) -> bool;
}
