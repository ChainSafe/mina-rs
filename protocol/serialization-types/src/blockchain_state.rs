// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::v1::{BlockTimeV1, ByteVecV1, Hash2V1, HashV1, TokenIdV1};
use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// Mina blockchain state struct
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

/// Staged ledger hash structure
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StagedLedgerHash {
    pub non_snark: NonSnarkStagedLedgerHashV1,
    pub pending_coinbase_hash: Hash2V1,
}

/// Staged ledger hash structure (v1)
pub type StagedLedgerHashV1 = Versioned<Versioned<Versioned<StagedLedgerHash, 1>, 1>, 1>;

/// Non-snarked ledger hash
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NonSnarkStagedLedgerHash {
    pub ledger_hash: HashV1,
    pub aux_hash: ByteVecV1,
    pub pending_coinbase_aux: ByteVecV1,
}

/// Non-snarked ledger hash (v1)
pub type NonSnarkStagedLedgerHashV1 = Versioned<NonSnarkStagedLedgerHash, 1>;
