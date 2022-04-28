// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the EpochData structure

use crate::numbers::{Amount, Length};
use mina_crypto::hash::*;
use proof_systems::mina_hasher::{Hashable, ROInput};

#[derive(Clone, Default, PartialEq, Debug)]
/// Epoch Ledger
pub struct EpochLedger {
    /// A unique identifier of the EpochLedger
    pub hash: LedgerHash,
    /// The total currency in circulation after the block was produced. New issuance is via the coinbase reward and new account fees can reduce the total issuance.
    pub total_currency: Amount,
}

impl Hashable for EpochLedger {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.hash);
        roi.append_hashable(&self.total_currency);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
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

impl Hashable for EpochData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.ledger);
        roi.append_hashable(&self.seed);
        roi.append_hashable(&self.start_checkpoint);
        roi.append_hashable(&self.lock_checkpoint);
        roi.append_hashable(&self.epoch_length);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
