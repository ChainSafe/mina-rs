// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

use crate::numbers::{BlockTime, TokenId};
use mina_crypto::hash::*;

#[derive(Clone, Default, Debug, PartialEq)]
/// Mina blockchain state struct
pub struct BlockchainState {
    /// Hash of the proposed next state of the blockchain
    pub staged_ledger_hash: StagedLedgerHash,
    /// Hash of the most recently proven state of the blockchain
    pub snarked_ledger_hash: SnarkedLedgerHash,
    /// Hash of the genesis state
    pub genesis_ledger_hash: SnarkedLedgerHash,
    /// Check whether the change of the next token ID is consistent.
    pub snarked_next_available_token: TokenId,
    /// Timestamps for blocks
    pub timestamp: BlockTime,
}
