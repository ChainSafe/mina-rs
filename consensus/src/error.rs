// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Types that represent errors in mina consensus operations
//!

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
}
