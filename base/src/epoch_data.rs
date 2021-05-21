
use crate::{
	numbers::{Amount, Length},
	hash::{StateHash, EpochSeed, LedgerHash},
};

#[derive(Hash)]
pub struct EpochLedger {
	hash: LedgerHash,
	total_currency: Amount
}

#[derive(Hash)]
pub struct EpochData {
	ledger: EpochLedger,
	seed: EpochSeed,
	start_checkpoint: StateHash,
	lock_checkpoint: StateHash,
	epoch_length: Length
}
