// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ConsensusError {
    #[error("header must have height 1 greater than top")]
    InvalidHeight,
    #[error("Top block not found")]
    TopBlockNotFound,
    #[error("Global slot not found")]
    GlobalSlotNotFound,
    #[error("Consensus state not found")]
    ConsensusStateNotFound,
    #[error("candidates not found")]
    CandidatesNotFound,
}
