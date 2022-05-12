// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::common::*;
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// Mina blockchain state struct
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockchainState {
    /// Hash of the proposed next state of the blockchain
    pub staged_ledger_hash: StagedLedgerHashV1,
    /// Hash of the most recently proven state of the blockchain
    pub snarked_ledger_hash: HashV1,
    /// Hash of the genesis state
    pub genesis_ledger_hash: HashV1,
    /// Check whether the change of the next token ID is consistent.
    pub snarked_next_available_token: TokenIdV1,
    /// Timestamps for blocks
    pub timestamp: BlockTimeV1,
}

/// Mina blockchain state struct (v1)
pub type BlockchainStateV1 = Versioned<Versioned<BlockchainState, 1>, 1>;

/// Mina blockchain state struct (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(BlockchainState)]
pub struct BlockchainStateJson {
    /// Hash of the proposed next state of the blockchain
    pub staged_ledger_hash: StagedLedgerHashJson,
    /// Hash of the most recently proven state of the blockchain
    pub snarked_ledger_hash: LedgerHashV1Json,
    /// Hash of the genesis state
    pub genesis_ledger_hash: LedgerHashV1Json,
    /// Check whether the change of the next token ID is consistent.
    pub snarked_next_available_token: U64Json,
    /// Timestamps for blocks
    pub timestamp: U64Json,
}

/// Staged ledger hash structure
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StagedLedgerHash {
    pub non_snark: NonSnarkStagedLedgerHashV1,
    pub pending_coinbase_hash: Hash2V1,
}

/// Staged ledger hash structure (v1)
pub type StagedLedgerHashV1 = Versioned<Versioned<StagedLedgerHash, 1>, 1>;

/// Staged ledger hash structure (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(StagedLedgerHash)]
pub struct StagedLedgerHashJson {
    pub non_snark: NonSnarkStagedLedgerHashJson,
    pub pending_coinbase_hash: CoinBaseHashV1Json,
}

/// Non-snarked ledger hash
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NonSnarkStagedLedgerHash {
    pub ledger_hash: HashV1,
    pub aux_hash: ByteVecV1,
    pub pending_coinbase_aux: ByteVecV1,
}

/// Non-snarked ledger hash (v1)
pub type NonSnarkStagedLedgerHashV1 = Versioned<NonSnarkStagedLedgerHash, 1>;

/// Non-snarked ledger hash (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(NonSnarkStagedLedgerHash)]
pub struct NonSnarkStagedLedgerHashJson {
    pub ledger_hash: LedgerHashV1Json,
    pub aux_hash: AuxHashJson,
    pub pending_coinbase_aux: PendingCoinbaseAuxHashJson,
}
