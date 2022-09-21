// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

use std::convert::TryInto;

use crate::{
    blockchain_state_registers::BlockchainStateRegisters,
    numbers::{BlockTime, TokenId},
    *,
};
use mina_crypto::hash::*;
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    mina_hasher::{Hashable, ROInput},
    ChunkedROInput, ToChunkedROInput,
};
use versioned::*;

#[derive(Clone, Default, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::blockchain_state::BlockchainState)]
/// Mina blockchain state struct (legacy)
pub struct BlockchainStateLegacy {
    /// Hash of the proposed next state of the blockchain
    pub staged_ledger_hash: StagedLedgerHash,
    /// Hash of the most recently proven state of the blockchain
    pub snarked_ledger_hash: LedgerHash,
    /// Hash of the genesis state
    pub genesis_ledger_hash: LedgerHash,
    /// Check whether the change of the next token ID is consistent.
    pub snarked_next_available_token: TokenId,
    /// Timestamps for blocks
    pub timestamp: BlockTime,
}

impl_from_with_proxy!(
    BlockchainStateLegacy,
    BlockchainStateV1,
    BlockchainStateJson
);

impl Hashable for BlockchainStateLegacy {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.staged_ledger_hash)
            .append_hashable(&self.snarked_ledger_hash)
            .append_hashable(&self.genesis_ledger_hash)
            .append_hashable(&self.snarked_next_available_token)
            .append_hashable(&self.timestamp)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
/// Mina blockchain state struct
pub struct BlockchainState {
    /// Hash of the proposed next state of the blockchain
    pub staged_ledger_hash: StagedLedgerHash,
    /// Hash of the genesis state
    pub genesis_ledger_hash: LedgerHash,
    /// Registers
    pub registers: BlockchainStateRegisters,
    /// Timestamps for blocks
    pub timestamp: BlockTime,
    /// Body reference
    pub body_reference: BodyReference,
}

impl FromGraphQLJson for BlockchainState {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self> {
        Ok(Self {
            staged_ledger_hash: StagedLedgerHash {
                non_snark: NonSnarkStagedLedgerHash {
                    ledger_hash: (&json["stagedLedgerHash"]).try_into()?,
                    aux_hash: (&json["stagedLedgerAuxHash"]).try_into()?,
                    pending_coinbase_aux: (&json["stagedLedgerPendingCoinbaseAux"]).try_into()?,
                },
                pending_coinbase_hash: (&json["stagedLedgerPendingCoinbaseHash"]).try_into()?,
            },
            // FIXME: missing from graphql API
            genesis_ledger_hash: (&json["genesisLedgerHash"]).try_into()?,
            registers: BlockchainStateRegisters {
                ledger: (&json["snarkedLedgerHash"]).try_into()?,
                pending_coinbase_stack: (),
                local_state: Default::default(),
            },
            timestamp: BlockTime(json["utcDate"].as_str().unwrap_or_default().parse()?),
            body_reference: BodyReference::from_hex(
                json["bodyReference"].as_str().unwrap_or_default(),
            )?,
        })
    }
}

impl ToChunkedROInput for BlockchainState {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.staged_ledger_hash)
            .append_chunked(&self.genesis_ledger_hash)
            .append_chunked(&self.registers)
            .append_chunked(&self.timestamp)
            .append_chunked(&self.body_reference)
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
/// Mina block body reference, wrapper of blake2 256-bit hash
pub struct BodyReference(pub [u8; 32]);

impl BodyReference {
    /// Construct [BodyReference] from hex str
    pub fn from_hex(data: impl AsRef<[u8]>) -> Result<Self, hex::FromHexError> {
        let mut r = [0; 32];
        hex::decode_to_slice(data, &mut r as &mut [u8])?;
        Ok(Self(r))
    }
}

impl ToChunkedROInput for BodyReference {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new().append_bytes(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_state_from_graphql_json() -> anyhow::Result<()> {
        const JSON_STR: &str = r###"
        {
            "bodyReference": "36bda176656cc3be96c3d317db7b4ac06fdbc7f4eedcd6efdd20e28143d67421",
            "date": "1655755201000",
            "snarkedLedgerHash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
            "stagedLedgerAuxHash": "UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom",
            "stagedLedgerHash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
            "stagedLedgerPendingCoinbaseHash": "2n27mUhCEctJbiZQdrk3kxYc7DVHvJVDErjXrjNs7jnP3HMLKtuN",
            "stagedLedgerPendingCoinbaseAux": "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
            "stagedLedgerProofEmitted": false,
            "utcDate": "1655755201000",
            "genesisLedgerHash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4"
        }
        "###;
        let json = serde_json::from_str(JSON_STR)?;
        _ = BlockchainState::from_graphql_json(&json)?;
        Ok(())
    }
}
