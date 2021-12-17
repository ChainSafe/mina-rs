// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

use crate::{numbers::BlockTime, token_id::TokenId};
use mina_crypto::hash::{SnarkedLedgerHash, StagedLedgerHash};
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
/// Mina blockchain state struct
#[wire_type(recurse = 2)]
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
