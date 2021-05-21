
use serde::{Serialize, Deserialize};

use crate::{
	numbers::BlockTime,
	token_id::TokenId,
	hash::{SnarkedLedgerHash, StagedLedgerHash},
};

#[derive(Serialize, Deserialize)]
pub struct BlockchainState {
    staged_ledger_hash: StagedLedgerHash,
    snarked_ledger_hash: SnarkedLedgerHash,
    genesis_ledger_hash: SnarkedLedgerHash,
    snarked_next_available_token: TokenId,
    timestamp: BlockTime,
}
