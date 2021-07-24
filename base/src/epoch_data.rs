// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::numbers::{Amount, Length};
use mina_crypto::hash::{EpochSeed, LedgerHash, StateHash};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct EpochLedger {
    hash: LedgerHash,
    total_currency: Amount,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct EpochData {
    ledger: EpochLedger,
    seed: EpochSeed,
    /// State hash of first block of epoch
    start_checkpoint: StateHash,
    /// State hash of last known block in the first 2/3 of epoch
    pub lock_checkpoint: StateHash,
    epoch_length: Length,
}
