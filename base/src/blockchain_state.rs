// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

use crate::{
    blockchain_state_registers::BlockchainStateRegisters,
    numbers::{BlockTime, TokenId},
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
// #[auto_from(mina_serialization_types::blockchain_state::BlockchainState)]
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
// #[auto_from(mina_serialization_types::blockchain_state::BlockchainState)]
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
