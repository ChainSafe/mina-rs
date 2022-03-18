// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A genesis ledger backed by a Rocksdb Instance
//! This is how they are provided by Mina

use crate::genesis_ledger::GenesisLedger;
use bin_prot::from_reader;
use mina_rs_base::account::Account;
use rocksdb::DB;

/// The first byte of keys in the RocksDB stored Ledger
/// that indicates the value is an Account (leaf node)
const ACCOUNT_PREFIX: u8 = 0xfe;

/// A genesis ledger backed by a RocksDB instance
pub struct RocksDbGenesisLedger<'a, const DEPTH: usize> {
    db: &'a DB,
}

impl<'a, const DEPTH: usize> RocksDbGenesisLedger<'a, DEPTH> {
    /// Create a new rocksDB genesis ledger given a database connection
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }
}

fn decode_account_from_kv((_k, v): (Box<[u8]>, Box<[u8]>)) -> Account {
    let account = from_reader(&v[..]).unwrap();
    println!("{:#?}", account);
    account
}

impl<'a, const DEPTH: usize> IntoIterator for &RocksDbGenesisLedger<'a, DEPTH> {
    type Item = Account;
    type IntoIter = Box<dyn Iterator<Item = Account> + 'a>;

    fn into_iter(self) -> Box<dyn Iterator<Item = Account> + 'a> {
        // A RockDB genesis ledger contains a Merkle tree. We only need to
        // iterate the leaves of the tree to iterate over all accounts.
        // It uses mina merkle_ledger locations as keys in the database
        // https://github.com/MinaProtocol/mina/blob/65b59f56b6e98e1d9648280c2153d809abb42ba3/src/lib/merkle_ledger/location.ml#L29
        // The first byte indicates the type of a record (e.g. internal node, leaf node) and
        // we can filter on this byte to only iterate the leaves

        let db_iter = self
            .db
            .prefix_iterator(&[ACCOUNT_PREFIX]) // This will ensure the iterator doesnt start until the prefix byte is matched
            .take_while(|(k, _)| k.first() == Some(&ACCOUNT_PREFIX)); // Ensures the iterator stops when the prefix stops matching

        Box::new(db_iter.map(decode_account_from_kv))
    }
}

impl<'a, const DEPTH: usize> GenesisLedger<'a, DEPTH> for RocksDbGenesisLedger<'a, DEPTH> {}

#[cfg(test)]
mod tests {
    use super::*;
    use rocksdb::Options;

    const DBPATH: &str =  "test-data/genesis_ledger_6a887ea130e53b06380a9ab27b327468d28d4ce47515a0cc59759d4a3912f0ef/";

    #[test]
    fn test_iterate_database() {
        let db = rocksdb::DB::open_for_read_only(&Options::default(), DBPATH, true).unwrap();
        let genesis_ledger: RocksDbGenesisLedger<20> = RocksDbGenesisLedger::new(&db);
        let accounts: Vec<_> = genesis_ledger.accounts().collect();
        assert_eq!(accounts.len(), 1676); // successfully read all the accounts
    }
}
