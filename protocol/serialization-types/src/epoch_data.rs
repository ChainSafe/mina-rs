// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the EpochData structure

use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};

use crate::{
    common::{U32, U64},
    json::*,
    v1::*,
};
use versioned::*;

/// Epoch Ledger
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EpochLedger {
    /// A unique identifier of the EpochLedger
    pub hash: HashV1,
    /// The total currency in circulation after the block was produced. New issuance is via the coinbase reward and new account fees can reduce the total issuance.
    pub total_currency: AmountV1,
}

/// Epoch Ledger (v1)
pub type EpochLedgerV1 = Versioned<Versioned<EpochLedger, 1>, 1>;

/// Epoch Ledger (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(EpochLedger)]
pub struct EpochLedgerJson {
    /// A unique identifier of the EpochLedger
    pub hash: LedgerHashV1Json,
    /// The total currency in circulation after the block was produced. New issuance is via the coinbase reward and new account fees can reduce the total issuance.
    pub total_currency: U64,
}

/// Epoch data
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

/// Epoch data (v1)
pub type EpochDataV1 = Versioned<Versioned<EpochData, 1>, 1>;

/// Epoch data (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(EpochData)]
pub struct EpochDataJson {
    /// Epoch Ledger, contains ledger related data for the epoch
    pub ledger: EpochLedgerJson,
    ///  Initialize the random number generator
    pub seed: EpochSeedHashV1Json,
    /// State hash of first block of epoch
    pub start_checkpoint: StateHashV1Json,
    /// State hash of last known block in the first 2/3 of epoch (excluding the current state)
    pub lock_checkpoint: StateHashV1Json,
    /// Length of an epoch
    pub epoch_length: U32,
}
