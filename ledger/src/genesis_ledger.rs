// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A Genesis ledger is specialization of a ledger that holds initial account data to be included at genesis
//!
//! A genesis ledger is the only way time locked accounts can be added to the ledger and
//! is also a way to initially allocate funds to accounts
//!

use mina_merkle::*;
use mina_rs_base::account::Account;

/// Type alias for mina merkle ledger hasher
pub type MinaLedgerMerkleHasher = MinaPoseidonMerkleHasher<Account>;

/// Type alias for mina merkle ledger
pub type MinaLedgerMerkleTree = MinaMerkleTree<
    <MinaLedgerMerkleHasher as MerkleHasher>::Item,
    <MinaLedgerMerkleHasher as MerkleHasher>::Hash,
    MinaLedgerMerkleHasher,
    MinaPoseidonMerkleMerger,
    FixedHeightMode,
>;

/// A genesis ledger provides access to its accounts by implementing IntoIterator
/// This implementation must be provided to meet the trait requirements
///
/// A Genesis ledger has a compile time pre-defined depth which is set here as a const generic
/// This ensures compile-time checking that the correct depth ledger is being used in the correc place
pub trait GenesisLedger<'a, const DEPTH: usize>
where
    Self: 'a,
    &'a Self: IntoIterator<Item = Result<Account, Self::Error>>,
{
    /// Error type that can be produces when trying to access the underlying store
    type Error;

    /// Return the depth of the ledger
    fn depth(&self) -> usize {
        DEPTH
    }

    /// Return a iterator over the accounts in this genesis ledger without consuming self
    fn accounts(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    /// Build mina merkle ledger tree with a fixed height
    fn to_mina_merkle_ledger(&'a self) -> MinaLedgerMerkleTree {
        const MINA_LEDGER_HEIGHT: u32 = 20;

        let mut tree = MinaLedgerMerkleTree::new(MINA_LEDGER_HEIGHT);
        tree.add_batch(self.accounts().flatten());
        tree
    }

    // TODO: Add additional methods when they are required
    // https://github.com/MinaProtocol/mina/blob/65b59f56b6e98e1d9648280c2153d809abb42ba3/src/lib/genesis_ledger/intf.ml#L84
}
