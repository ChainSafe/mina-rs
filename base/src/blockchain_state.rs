// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    hash::{SnarkedLedgerHash, StagedLedgerHash},
    numbers::BlockTime,
    token_id::TokenId,
};

#[derive(Serialize, Deserialize)]
pub struct BlockchainState {
    staged_ledger_hash: StagedLedgerHash,
    snarked_ledger_hash: SnarkedLedgerHash,
    genesis_ledger_hash: SnarkedLedgerHash,
    snarked_next_available_token: TokenId,
    timestamp: BlockTime,
}
