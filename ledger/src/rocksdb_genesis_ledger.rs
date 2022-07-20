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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    use ark_ff::*;
    use mina_consensus::genesis::Genesis;
    use mina_crypto::hash::*;
    use mina_merkle::*;
    use mina_rs_base::{
        account::{Account, AccountLegacy},
        types::ExternalTransition,
    };
    use num::BigUint;
    use pretty_assertions::{assert_eq, assert_ne};
    use proof_systems::{
        mina_hasher::{self, Fp, Hasher, ROInput},
        mina_signer::CompressedPubKey,
    };
    use rocksdb::*;

    #[test]
    fn test_iterate_database() {
        const DBPATH: &str =  "test-data/genesis_ledger_6a887ea130e53b06380a9ab27b327468d28d4ce47515a0cc59759d4a3912f0ef/";
        let db = rocksdb::DB::open_for_read_only(&Options::default(), DBPATH, true).unwrap();
        let genesis_ledger: RocksDbGenesisLedger<20, AccountLegacy> =
            RocksDbGenesisLedger::new(&db);
        let accounts: Vec<_> = genesis_ledger.accounts().collect();
        assert_eq!(accounts.len(), 1676); // successfully read the correct number of accounts

        let mut expected_account_hashes = Vec::with_capacity(accounts.len());
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
            let account = account.unwrap();
            let hash = MinaLedgerMerkleHasher::hash(&account, MerkleTreeNodeMetadata::new(0, 1));
            let hash_expected = expected_account_hashes[i];
            // TODO: Change this to assert_eq! when Hashable is completely implemented for Account
            assert_ne!(hash, hash_expected);
        }
    }

    #[test]
    fn test_iterate_database_berkeley() {
        const DBPATH: &str =  "test-data/genesis_ledger_266b7c62f51cf5ac895e98e681cc34bc39e5c29ee79ac069fb399c022fc5d1c4/";
        let db = rocksdb::DB::open_for_read_only(&Options::default(), DBPATH, true).unwrap();
        let genesis_ledger: RocksDbGenesisLedger<20, Account> = RocksDbGenesisLedger::new(&db);
        let accounts: Vec<_> = genesis_ledger.accounts().collect();
        assert_eq!(accounts.len(), 6404); // successfully read the correct number of accounts

        let mut expected_account_hashes = Vec::with_capacity(accounts.len());
        let mut expected_root_height = 0;
        let mut expected_root_hash: Option<Fp> = None;
        for (key, value) in db
            .iterator(IteratorMode::Start)
            .take_while(|(key, _)| key[0] < 0xfe)
        {
            let height = key[0];
            let hash: Fp = BigInteger256::read(&value[..]).unwrap().into();
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
        // FIXME: Use genesis block from hard fork instead
        // let ledger_hash = LedgerHash::try_from(&expected_root_hash.unwrap()).unwrap();
        // let genesis_block =
        //     ExternalTransition::from_genesis_config(&mina_consensus::genesis::MAINNET_CONFIG);
        // assert_eq!(
        //     ledger_hash,
        //     genesis_block
        //         .protocol_state
        //         .body
        //         .blockchain_state
        //         .genesis_ledger_hash
        // );
        // TODO: Change this to assert_eq! when Hashable is completely implemented for Account
        assert_ne!(merkle_ledger.root(), expected_root_hash);
        assert_eq!(accounts.len(), expected_account_hashes.len());
        for (i, account) in accounts.into_iter().enumerate() {
            let account = account.unwrap();
            let hash = MinaLedgerMerkleHasher::hash(&account, MerkleTreeNodeMetadata::new(0, 1));
            let hash_expected = expected_account_hashes[i];
            // Test account 0
            if i == 0 {
                #[derive(Debug, Clone)]
                struct CompressedPubKeyHashableWrapper<'a>(pub &'a CompressedPubKey);

                impl<'a> Hashable for CompressedPubKeyHashableWrapper<'a> {
                    type D = ();

                    fn to_roinput(&self) -> ROInput {
                        let mut roi = ROInput::new();
                        roi.append_field(self.0.x);
                        roi.append_bool(self.0.is_odd);
                        roi
                    }

                    fn domain_string(_: Self::D) -> Option<String> {
                        None
                    }
                }

                fn fp_to_big(fp: Fp) -> BigUint {
                    let big256: BigInteger256 = fp.into();
                    big256.into()
                }

                fn hash<T: Hashable<D = ()>>(t: &T) -> String {
                    let mut hasher = mina_hasher::create_kimchi(());
                    let fp = hasher.hash(t);
                    fp_to_big(fp).to_str_radix(10)
                }
                println!("{}", account.public_key.into_address());
                assert_eq!(
                    hash(&CompressedPubKeyHashableWrapper(&account.public_key)),
                    "17403802830378787968845294854048648555868428232350653563068266009233402282076"
                );
                assert_eq!(
                    hash(&account.token_id),
                    "7555220006856562833147743033256142154591945963958408607501861037584894828141"
                );
                assert_eq!(
                    hash(&account.balance),
                    "9880909019220052913227433707787222982896169056561545508056145968396555243660"
                );
                assert_eq!(
                    hash(&account.token_permissions),
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                );
                assert_eq!(
                    hash(&account.token_symbol),
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                );
                assert_eq!(
                    hash(&account.nonce),
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                );
                assert_eq!(
                    hash(&account.receipt_chain_hash),
                    "21992065535400692533677074789790277789989066181791602188282189650879541934688"
                );
                match &account.delegate {
                    Some(delegate) => {
                        assert_eq!(
                            hash(&CompressedPubKeyHashableWrapper(delegate)),
                            "17403802830378787968845294854048648555868428232350653563068266009233402282076"
                        );
                    }
                    None => {
                        assert!(false);
                    }
                }
                assert_eq!(
                    hash(&account.voting_for),
                    "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                );
                let roi = account.timing.to_roinput();
                println!("timing: {:?}", account.timing);
                for f in roi.to_fields() {
                    println!(" {}", fp_to_big(f));
                }
                assert_eq!(
                    hash(&account.timing),
                    "7555220006856562833147743033256142154591945963958408607501861037584894828141"
                );
            }

            // TODO: Change this to assert_eq! when Hashable is completely implemented for Account
            assert_eq!(
                hash,
                hash_expected,
                "{} != {}",
                StateHash::from(&hash),
                StateHash::from(&hash_expected)
            );
        }
    }
}
