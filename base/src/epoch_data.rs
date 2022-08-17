// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the EpochData structure

use crate::numbers::{Amount, Length};
use mina_crypto::hash::*;
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    mina_hasher::{Hashable, ROInput},
    ChunkedROInput, ToChunkedROInput,
};

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::epoch_data::EpochLedger)]
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
        ROInput::new()
            .append_hashable(&self.hash)
            .append_hashable(&self.total_currency)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for EpochLedger {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
    }
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::epoch_data::EpochData)]
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
        ROInput::new()
            .append_hashable(&self.seed)
            .append_hashable(&self.start_checkpoint)
            .append_hashable(&self.epoch_length)
            .append_hashable(&self.ledger)
            .append_hashable(&self.lock_checkpoint)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
