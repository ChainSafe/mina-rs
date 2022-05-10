// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Traits and helpers for data structures that require validation

/// Types that implement Verifiable are able to self-check using intrinsic data (e.g. signatures)
/// and optionally some context that is required for peforming the verification
/// e.g. a singleton signature verifier or param required for snark verification
pub trait Verifiable<CTX> {
    /// Return if the implementor is valid
    fn verify(&self, ctx: CTX) -> bool;
}
