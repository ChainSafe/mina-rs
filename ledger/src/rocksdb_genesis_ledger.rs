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

    use anyhow::bail;
    use ark_ff::*;
    use mina_consensus::genesis::Genesis;
    use mina_crypto::hash::*;
    use mina_merkle::*;
    use mina_rs_base::{account::*, types::*};
    use num::BigUint;
    use pretty_assertions::{assert_eq, assert_ne};
    use proof_systems::{
        mina_hasher::{self, Fp, Hasher, ROInput},
        ToChunkedROInput,
    };
    use rocksdb::*;

    const DBPATH_LEGACY: &str =  "test-data/genesis_ledger_6a887ea130e53b06380a9ab27b327468d28d4ce47515a0cc59759d4a3912f0ef/";
    const DBPATH: &str =  "test-data/genesis_ledger_266b7c62f51cf5ac895e98e681cc34bc39e5c29ee79ac069fb399c022fc5d1c4/";

    #[test]
    fn test_iterate_database() -> anyhow::Result<()> {
        let db = rocksdb::DB::open_for_read_only(&Options::default(), DBPATH_LEGACY, true)?;
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
            let hash: Fp = BigInteger256::read(&value[2..])?.into();
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
        let ledger_hash = LedgerHash::try_from(&expected_root_hash.unwrap())?;
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
            let account = account?;
            let hash = MinaLedgerMerkleHasher::hash(&account, MerkleTreeNodeMetadata::new(0, 1));
            let hash_expected = expected_account_hashes[i];
            // TODO: Change this to assert_eq! when Hashable is completely implemented for Account
            assert_ne!(hash, hash_expected);
        }
        Ok(())
    }

    #[test]
    fn test_iterate_database_berkeley() -> anyhow::Result<()> {
        let db = rocksdb::DB::open_for_read_only(&Options::default(), DBPATH, true)?;
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
            let hash: Fp = BigInteger256::read(&value[..])?.into();
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
        // let ledger_hash = LedgerHash::try_from(&expected_root_hash?)?;
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

        assert_ne!(merkle_ledger.root(), expected_root_hash);
        assert_eq!(accounts.len(), expected_account_hashes.len());
        for (i, account) in accounts.into_iter().enumerate() {
            let account = account?;
            let hash =
                MinaLedgerKimchiMerkleHasher::hash(&account, MerkleTreeNodeMetadata::new(0, 1));
            let hash_expected = expected_account_hashes[i];

            assert_eq!(
                hash,
                hash_expected,
                "account {i}: {} != {}",
                StateHash::from(&hash),
                StateHash::from(&hash_expected)
            );
        }
        Ok(())
    }

    /// Some useful OCaml code snippet for debugging
    ///
    /// To print the n-th account in genesis ledger, add this to `inputs_from_config_file` in `genesis_ledger_helper.ml`
    /// ```ocaml
    /// let accounts = genesis_ledger |> Genesis_ledger.Packed.accounts |> Lazy.force in
    ///   let n_accounts = accounts |> List.length in
    ///   [%log info] "genesis_ledger n_accounts: %d" (n_accounts);
    ///   let acc_n = match List.nth accounts 1 with
    ///     | Some (_, acc) -> acc
    ///     | None -> (Core.exit 1) in
    ///   [%log info] "genesis_ledger acc_n: $acc_n" ~metadata: [
    ///     ("acc_n", acc_n |> Account.to_yojson)];
    ///   [%log info] "genesis_ledger acc0 hash: $hash" ~metadata: [
    ///     ("acc_n hash", acc_n |> Account.crypto_hash |> State_hash.to_yojson)];
    /// ```
    ///
    /// To print inner value of random oracle input, add this to `random_oracle_iinput.ml`
    /// ```ocaml
    /// let print (a : _ t) field_to_str =
    /// printf "field_elements:\n";
    /// for i = 0 to Array.length a.field_elements - 1 do
    ///   printf "%d field_elements: %s\n" i (a.field_elements.(i) |> field_to_str)
    /// done;
    /// printf "packed:\n";
    /// for i = 0 to Array.length a.packeds - 1 do
    ///   let (f, len) = a.packeds.(i) in
    ///   printf "%d packed (%d): %s \n" i len (f |> field_to_str)
    // done
    /// ```
    ///
    /// To skip fields in account, replace `f` with below `f2` in `account.ml`
    /// ```ocaml
    /// let f2 mk acc field =
    ///     (* printf "f2:%s\n" (Core_kernel.Field.name field); *)
    ///     let _ignore = mk (Core_kernel.Field.get field t) in
    ///     acc
    ///   in
    /// ```
    ///
    /// To print account hash result, add this to `genesis_ledger_helper.ml`
    /// ```ocaml
    /// let%test_unit "genesis ledger acc_n" =
    ///   let acc_json =
    ///   In_channel.with_file "/path-to/account.json" ~f:(fun in_channel ->
    ///       try Yojson.Safe.from_channel in_channel with
    ///       | _ ->
    ///           Core.exit 1) in
    ///   let acc = match acc_json |> Account.of_yojson with
    ///       | Ok acc -> acc
    ///       | Error _ -> Core.exit 1
    ///   in
    ///   let hash = acc |> Account.crypto_hash in
    ///   printf "account hash:%s\n%s\n"
    ///   (hash |> State_hash.to_yojson |> Yojson.Safe.to_string)
    ///   (hash |> Snark_params.Tick.Field.to_string);
    ///   (* Comment out fields in Account to verify hashes of every single field *)
    ///   let chunked_input = acc |> Account.to_input in
    ///   Random_oracle_input.Chunked.print chunked_input Snark_params.Tick.Field.to_string;
    ///   let debug_input = chunked_input |> Random_oracle.pack_input in
    ///   printf "debug_input fields:\n";
    ///   for i = 0 to Array.length debug_input - 1 do
    ///     printf "debug_input:\"%s\",\n" (debug_input.(i) |> Snark_params.Tick.Field.to_string)
    ///   done ;
    ///   let debug_hash = debug_input |> Random_oracle.hash |> Snark_params.Tick.Field.to_string in
    ///   printf "account hash:%s\naccount debug hash:%s\n" (hash |> Snark_params.Tick.Field.to_string) debug_hash
    /// ```
    #[test]
    fn test_genesis_ledger_account_0() -> anyhow::Result<()> {
        const N: usize = 0;
        let account = get_nth_account(DBPATH, N)?;
        let expected_hash = get_nth_hash(DBPATH, N)?;

        assert_eq!(
            hash2(&CompressedPubKeyHashableWrapper(&account.public_key)),
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
            hash2(&account.token_permissions),
            "21565680844461314807147611702860246336805372493508489110556896454939225549736"
        );
        assert_eq!(
            hash2(&account.token_symbol),
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
                    hash2(&CompressedPubKeyHashableWrapper(delegate)),
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
        assert_eq!(
            hash2(&account.timing),
            "7555220006856562833147743033256142154591945963958408607501861037584894828141"
        );
        assert_eq!(
            hash(&ZkAppOptionHashableWrapper(&account.zkapp)),
            "22371316807638652529772065903909764704228252716310880671193348070876705445596"
        );
        assert_eq!(
            hash2(&account.permissions),
            "17687022753513245123643156797999811582870093245402815918931465038658213870633"
        );
        assert_eq!(
            hash(&ZkAppUriOptionHashableWrapper(&account.zkapp_uri)),
            "20639848968581348850513072699760590695338607317404146322838943866773129280073"
        );
        assert_eq!(hash(&account), fp_to_big(expected_hash).to_str_radix(10),);
        Ok(())
    }

    fn get_nth_account(db_path: impl AsRef<str>, n: usize) -> anyhow::Result<Account> {
        let db = rocksdb::DB::open_for_read_only(&Options::default(), db_path.as_ref(), true)?;
        let genesis_ledger: RocksDbGenesisLedger<20, Account> = RocksDbGenesisLedger::new(&db);
        let r = if let Some(Ok(account)) = genesis_ledger.accounts().nth(n) {
            account
        } else {
            bail!("Fail to get {n}th account")
        };
        Ok(r)
    }

    fn get_nth_hash(db_path: impl AsRef<str>, n: usize) -> anyhow::Result<Fp> {
        let db = rocksdb::DB::open_for_read_only(&Options::default(), db_path.as_ref(), true)?;
        let r = if let Some((_, value)) = db
            .iterator(IteratorMode::Start)
            .take_while(|(key, _)| key[0] < 0xfe)
            .nth(n)
        {
            BigInteger256::read(&value[..])?.into()
        } else {
            bail!("Fail to get {n}th account")
        };
        Ok(r)
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

    fn hash2<T: ToChunkedROInput>(t: &T) -> String {
        #[derive(Debug, Clone)]
        struct ChunkedROInputHashableWrapper(ROInput);

        impl Hashable for ChunkedROInputHashableWrapper {
            type D = ();

            fn to_roinput(&self) -> ROInput {
                self.0.clone()
            }

            fn domain_string(_: Self::D) -> Option<String> {
                None
            }
        }

        hash(&ChunkedROInputHashableWrapper(t.roinput()))
    }
}
