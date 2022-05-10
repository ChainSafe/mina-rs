// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Traits and helpers for data structures that require validation

/// Types that implement Verifiable are able to self-check using intrinsic data (e.g. signatures)
/// and optionally some supplimentary data specific to the implementing type (e.g. CRS for snark proofs)
pub trait Verifiable {
    /// Supplimentary data type required to verify
    type Sup;

    /// Return if the implementor is valid
    fn verify(&self, data: Self::Sup) -> bool;
}
