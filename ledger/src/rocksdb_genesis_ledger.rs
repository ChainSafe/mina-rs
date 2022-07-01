// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A genesis ledger backed by a Rocksdb Instance
//! This is how they are provided by Mina

use crate::genesis_ledger::GenesisLedger;
use bin_prot::from_reader;
use mina_rs_base::{account::Account, *};
use rocksdb::DB;
use thiserror::Error;

/// The first byte of keys in the RocksDB stored Ledger
/// that indicates the value is an Account (leaf node)
const ACCOUNT_PREFIX: u8 = 0xfe;

/// A genesis ledger backed by a RocksDB instance
pub struct RocksDbGenesisLedger<'a, const DEPTH: usize> {
    db: &'a DB,
}

/// Errors than can be produces when trying to access a Rocksdb backed ledger
#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not deserialize account")]
    Disconnect(#[from] bin_prot::error::Error),
}

impl<'a, const DEPTH: usize> RocksDbGenesisLedger<'a, DEPTH> {
    /// Create a new rocksDB genesis ledger given a database connection
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }
}

fn decode_account_from_kv((_k, v): (Box<[u8]>, Box<[u8]>)) -> Result<Account, Error> {
    let account: <Account as BinProtSerializationType>::T = from_reader(&v[..])?;
    Ok(account.into())
}

impl<'a, const DEPTH: usize> IntoIterator for &RocksDbGenesisLedger<'a, DEPTH> {
    type Item = Result<Account, Error>;
    type IntoIter = Box<dyn Iterator<Item = Result<Account, Error>> + 'a>;

    fn into_iter(self) -> Box<dyn Iterator<Item = Result<Account, Error>> + 'a> {
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

impl<'a, const DEPTH: usize> GenesisLedger<'a, DEPTH> for RocksDbGenesisLedger<'a, DEPTH> {
    type Error = Error;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    use ark_ff::*;
    use mina_consensus::genesis::Genesis;
    use mina_crypto::hash::*;
    use mina_merkle::*;
    use mina_rs_base::types::ExternalTransition;
    use proof_systems::mina_hasher::Fp;
    use rocksdb::*;

    const DBPATH: &str =  "test-data/genesis_ledger_6a887ea130e53b06380a9ab27b327468d28d4ce47515a0cc59759d4a3912f0ef/";

    #[test]
    fn test_iterate_database() {
        let db = rocksdb::DB::open_for_read_only(&Options::default(), DBPATH, true).unwrap();
        let genesis_ledger: RocksDbGenesisLedger<20> = RocksDbGenesisLedger::new(&db);
        let accounts: Vec<_> = genesis_ledger.accounts().collect();
        assert_eq!(accounts.len(), 1676); // successfully read the correct number of accounts
        assert!(genesis_ledger.accounts().all(|e| e.is_ok())); // All deserialied sucessfully

        let mut expected_account_hashes = Vec::with_capacity(genesis_ledger.accounts().count());
        let mut expected_root_height = 0;
        let mut expected_root_hash: Option<Fp> = None;
        for (key, value) in db
            .iterator(IteratorMode::Start)
            .take_while(|(key, _)| key[0] < 0xfe)
        {
            let height = key[0];
            let hash: Fp = BigInteger256::read(&value[2..]).unwrap().into();
            if height > expected_root_height {
                expected_root_height = height;
                expected_root_hash = Some(hash);
            }
            if height == 0 {
                expected_account_hashes.push(hash);
            }
        }

        let mut merkle_ledger = genesis_ledger.to_mina_merkle_ledger();
        assert!(expected_root_hash.is_some());
        assert_eq!(merkle_ledger.height(), expected_root_height as u32);
        let ledger_hash = LedgerHash::try_from(&expected_root_hash.unwrap()).unwrap();
        let genesis_block =
            ExternalTransition::from_genesis_config(&mina_consensus::genesis::MAINNET_CONFIG);
        assert_eq!(
            ledger_hash,
            genesis_block
                .protocol_state
                .body
                .blockchain_state
                .genesis_ledger_hash
        );
        // TODO: Change this to assert_eq! when Hashable is completely implemented for Account
        assert_ne!(merkle_ledger.root(), expected_root_hash);
        assert_eq!(accounts.len(), expected_account_hashes.len());
        for (i, account) in accounts.into_iter().enumerate() {
            assert!(account.is_ok());
            let account = account.unwrap();
            let hash = MinaLedgerMerkleHasher::hash(&account, MerkleTreeNodeMetadata::new(0, 1));
            let hash_expected = expected_account_hashes[i];
            // TODO: Change this to assert_eq! when Hashable is completely implemented for Account
            assert_ne!(hash, hash_expected);
        }
    }
}
