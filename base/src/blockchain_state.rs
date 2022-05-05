// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State

use crate::numbers::{BlockTime, TokenId};
use mina_crypto::hash::*;
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_hasher::{Hashable, ROInput};
use versioned::*;

#[derive(Clone, Default, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::blockchain_state::BlockchainState)]
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

impl_from_with_proxy!(BlockchainState, BlockchainStateV1, BlockchainStateJson);

impl Hashable for BlockchainState {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.staged_ledger_hash);
        roi.append_hashable(&self.snarked_ledger_hash);
        roi.append_hashable(&self.genesis_ledger_hash);
        roi.append_hashable(&self.snarked_next_available_token);
        roi.append_hashable(&self.timestamp);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
