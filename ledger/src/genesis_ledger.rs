// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A Genesis ledger is specialization of a ledger that holds initial account data to be included at genesis
//!
//! A genesis ledger is the only way time locked accounts can be added to the ledger and
//! is also a way to initially allocate funds to accounts
//!

use mina_rs_base::account::Account;

/// A genesis ledger provides access to its accounts by implementing IntoIterator
/// This implementation must be provided to meet the trait requirements
pub trait GenesisLedger<'a>
where
    Self: 'a,
    &'a Self: IntoIterator<Item = Account>,
{
    /// Return the depth of the ledger
    fn depth(&self) -> u32;

    /// Return a iterator over the accounts in this genesis ledger without consuming self
    fn accounts(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
