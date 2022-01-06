// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::numbers::{Amount, Length};
use mina_crypto::hash::*;
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct EpochLedger {
    pub hash: LedgerHash,
    pub total_currency: Amount,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct EpochData {
    pub ledger: EpochLedger,
    pub seed: EpochSeed,
    /// State hash of first block of epoch
    pub start_checkpoint: StateHash,
    /// State hash of last known block in the first 2/3 of epoch
    pub lock_checkpoint: StateHash,
    pub epoch_length: Length,
}
