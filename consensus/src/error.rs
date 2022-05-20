// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Types that represent errors in mina consensus operations
//!

use std::str::Utf8Error;

/// Type that represents errors in mina consensus operations
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ConsensusError {
    /// header must have height 1 greater than top
    #[error("header must have height 1 greater than top")]
    InvalidHeight,

    /// Top block not found
    #[error("Top block not found")]
    TopBlockNotFound,

    /// Global slot not found
    #[error("Global slot not found")]
    GlobalSlotNotFound,

    /// Consensus state not found
    #[error("Consensus state not found")]
    ConsensusStateNotFound,

    /// candidates not found
    #[error("candidates not found")]
    CandidatesNotFound,
    /// Candidates missing some sub window densities
    #[error("Candidates missing some sub window densities")]
    CandidatesMissingSubWindowDensities,
    /// Invalid sub window density length
    #[error("Invalid sub window density length")]
    InvalidSubWindowDensityLen,

    /// Invalid sub window density length
    #[error("Could not generate blake2b digest of last vrf output: {0}")]
    FailedVrfHashDigest(Utf8Error),
}
