// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the EpochData structure

use crate::{
    numbers::{Amount, Length},
    *,
};
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

impl FromGraphQLJson for EpochLedger {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self> {
        Ok(Self {
            hash: LedgerHash::from_str(json["hash"].as_str().unwrap_or_default())?,
            total_currency: json["totalCurrency"]
                .as_str()
                .unwrap_or_default()
                .parse::<u64>()?
                .into(),
        })
    }
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
            .append_chunked(&self.hash)
            .append_chunked(&self.total_currency)
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

impl FromGraphQLJson for EpochData {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self> {
        Ok(Self {
            ledger: EpochLedger::from_graphql_json(&json["ledger"])?,
            seed: EpochSeed::from_str(json["seed"].as_str().unwrap_or_default())?,
            start_checkpoint: StateHash::from_str(
                json["startCheckpoint"].as_str().unwrap_or_default(),
            )?,
            lock_checkpoint: StateHash::from_str(
                json["lockCheckpoint"].as_str().unwrap_or_default(),
            )?,
            epoch_length: json["epochLength"]
                .as_str()
                .unwrap_or_default()
                .parse::<u32>()?
                .into(),
        })
    }
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

impl ToChunkedROInput for EpochData {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.seed)
            .append_chunked(&self.start_checkpoint)
            .append_chunked(&self.epoch_length)
            .append_chunked(&self.ledger)
            .append_chunked(&self.lock_checkpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_data_from_graphql_json() -> anyhow::Result<()> {
        const JSON_STR: &str = r###"
        {
            "epochLength": "2",
            "lockCheckpoint": "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
            "seed": "2vc1zQHJx2xN72vaR4YDH31KwFSr5WHSEH2dzcfcq8jxBPcGiJJA",
            "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
            "ledger": {
              "hash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
              "totalCurrency": "1013238001000001000"
            }
          }
        "###;
        let json = serde_json::from_str(JSON_STR)?;
        _ = EpochData::from_graphql_json(&json)?;
        Ok(())
    }
}
