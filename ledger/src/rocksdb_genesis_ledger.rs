// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A genesis ledger backed by a Rocksdb Instance
//! This is how they are provided by Mina

use crate::genesis_ledger::GenesisLedger;
use bin_prot::*;
use mina_rs_base::*;
use proof_systems::mina_hasher::Hashable;
use rocksdb::DB;
use std::marker::PhantomData;
use thiserror::Error;

/// The first byte of keys in the RocksDB stored Ledger
/// that indicates the value is an Account (leaf node)
const ACCOUNT_PREFIX: u8 = 0xfe;

/// A genesis ledger backed by a RocksDB instance
pub struct RocksDbGenesisLedger<
    'a,
    const DEPTH: usize,
    Account: Hashable + BinProtSerializationType<'a>,
> {
    db: &'a DB,
    _pd: PhantomData<Account>,
}

/// Errors that can be produces when trying to access a Rocksdb backed ledger
#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not deserialize account: {0}\nkey:{1:?}, value:{2:?}")]
    Disconnect(bin_prot::error::Error, Vec<u8>, Vec<u8>),
}

impl<'a, const DEPTH: usize, Account: Hashable + BinProtSerializationType<'a>>
    RocksDbGenesisLedger<'a, DEPTH, Account>
{
    /// Create a new rocksDB genesis ledger given a database connection
    pub fn new(db: &'a DB) -> Self {
        Self {
            db,
            _pd: Default::default(),
        }
    }
}

fn decode_account_from_kv<'a, Account: BinProtSerializationType<'a>>(
    (k, v): (Box<[u8]>, Box<[u8]>),
) -> Result<Account, Error> {
    let account: <Account as BinProtSerializationType>::T =
        from_reader_strict(&v[..]).map_err(|err| Error::Disconnect(err, k.to_vec(), v.to_vec()))?;
    Ok(account.into())
}

impl<'a, const DEPTH: usize, Account: Hashable + BinProtSerializationType<'a> + 'a> IntoIterator
    for &RocksDbGenesisLedger<'a, DEPTH, Account>
{
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

impl<'a, const DEPTH: usize, Account: Hashable + BinProtSerializationType<'a> + 'a>
    GenesisLedger<'a, DEPTH, Account> for RocksDbGenesisLedger<'a, DEPTH, Account>
where
    <Account as Hashable>::D: Default,
{
    type Error = Error;
}
