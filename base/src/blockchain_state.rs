// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::{numbers::BlockTime, token_id::TokenId};
use mina_crypto::hash::{SnarkedLedgerHash, StagedLedgerHash};
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct BlockchainState {
    pub staged_ledger_hash: StagedLedgerHash,
    pub snarked_ledger_hash: SnarkedLedgerHash,
    pub genesis_ledger_hash: SnarkedLedgerHash,
    pub snarked_next_available_token: TokenId,
    pub timestamp: BlockTime,
}
