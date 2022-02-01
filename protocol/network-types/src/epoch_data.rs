// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the EpochData structure

use serde::{Deserialize, Serialize};

use crate::v1::{AmountV1, HashV1, LengthV1};
use versioned::Versioned;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Epoch Ledger
pub struct EpochLedger {
    /// A unique identifier of the EpochLedger
    pub hash: HashV1,
    /// The total currency in circulation after the block was produced. New issuance is via the coinbase reward and new account fees can reduce the total issuance.
    pub total_currency: AmountV1,
}

pub type EpochLedgerV1 = Versioned<Versioned<EpochLedger, 1>, 1>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Epoch data
pub struct EpochData {
    /// Epoch Ledger, contains ledger related data for the epoch
    pub ledger: EpochLedgerV1,
    ///  Initialize the random number generator
    pub seed: HashV1,
    /// State hash of first block of epoch
    pub start_checkpoint: HashV1,
    /// State hash of last known block in the first 2/3 of epoch (excluding the current state)
    pub lock_checkpoint: HashV1,
    /// Length of an epoch
    pub epoch_length: LengthV1,
}

pub type EpochDataV1 = Versioned<Versioned<EpochData, 1>, 1>;
