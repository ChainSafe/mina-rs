//! A Genesis ledger is a read-only mapping of accounts to 
//! balances and optinally time-lock information. This is loaded into an actual ledger
//! which can be mutated by transactions.
//! 
//! A genesis ledger is the only way time locked accounts can be added to the ledger and 
//! is also a way to initially allocate funds to accounts
//!

use std::marker::PhantomData;

use crate::account::{Account, Timing};
use mina_rs_base::numbers::Amount;
use mina_crypto::signature::{PublicKey, PrivateKey, KeyPair};

pub struct PublicAccount {
	pk: PublicKey,
	balance: Amount,
	delegate: Option<PublicKey>,
	timing: Timing,
}

pub struct PrivateAccount {
	pk: PublicKey,
	sk: PrivateKey,
	balance: Amount,
	timing: Timing,
}

/// A genesis ledger provides access to its accounts by implementing IntoIterator
/// This implementation must be provided to meet the trait requirements
pub trait GenesisLedger<'a> where Self: 'a, &'a Self: IntoIterator<Item=Account> {
	
	/// Return a iterator over the accounts in this genesis ledger without consuming self
	fn accounts(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
		self.into_iter()
	}

	/// Return the length (how many accounts)
	fn depth(&self) -> u32;

	/// Retrieve the keypair of an account in the genesis ledger
	fn keypair_of_account_record(&self, account: Account) -> KeyPair;
}


pub trait GenesisLedgerBuilder<'a, T> where T:  GenesisLedger<'a>, &'a T: IntoIterator<Item=Account> {
	fn add_account(&mut self, account: Account);
	fn build(self) -> T;	
}


/// A genesis ledger backed by a Rocksdb Instance
/// This is how they are provided by Mina

use rocksdb::{DB, Direction, IteratorMode, DBIterator};

struct RocksDbGenesisLedger<'a> {
	db: &'a DB,
}

impl<'a> RocksDbGenesisLedger<'a> {
	pub fn new(db: &'a DB) -> Self {
		Self { db }
	}
}

fn decode_account_from_kv((k, v): (Box<[u8]>, Box<[u8]>)) -> Account {
	// TODO - write the logic for decoding an account from the db store
	Account::default()
}

impl<'a> IntoIterator for &RocksDbGenesisLedger<'a> {
	type Item = Account;
	type IntoIter = core::iter::Map<DBIterator<'a>, fn((Box<[u8]>, Box<[u8]>)) -> Account>;

	fn into_iter(self) -> Self::IntoIter {
	    let db_iter = self.db.iterator(IteratorMode::Start);
	    db_iter.map(decode_account_from_kv)
	}
}

impl<'a> GenesisLedger<'a> for RocksDbGenesisLedger<'a> {
	fn depth(&self) -> u32 {
	    unimplemented!()
	}

	fn keypair_of_account_record(&self, account: Account) -> KeyPair {
	    unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_iterate_database() {
		let db = rocksdb::DB::open_default("./genesis_ledger_accounts_71d4f4a0e8c1f3e78760b989234760584969ea0144ed1a3057234f6f0e73621a").unwrap();
		let genesis_ledger = RocksDbGenesisLedger::new(&db);
		for acc in genesis_ledger.into_iter() {
			println!("{:?}", acc);
		}

	}
}