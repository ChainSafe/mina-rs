// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the EpochData structure

use crate::numbers::{Amount, Length};
use mina_crypto::hash::*;

#[derive(Clone, Default, PartialEq, Debug)]
/// Epoch Ledger
pub struct EpochLedger {
    /// A unique identifier of the EpochLedger
    pub hash: LedgerHash,
    /// The total currency in circulation after the block was produced. New issuance is via the coinbase reward and new account fees can reduce the total issuance.
    pub total_currency: Amount,
}

#[derive(Clone, Default, PartialEq, Debug)]
/// Epoch data
pub struct EpochData {
    /// Epoch Ledger, contains ledger related data for the epoch
    pub ledger: EpochLedger,
    ///  Initialize the random number generator
    pub seed: EpochSeed,
    /// State hash of first block of epoch
    pub start_checkpoint: StateHash,
    /// State hash of last known block in the first 2/3 of epoch (excluding the current state)
    pub lock_checkpoint: StateHash,
    /// Length of an epoch
    pub epoch_length: Length,
}
