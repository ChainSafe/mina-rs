// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account Timing

use crate::numbers::{Amount, BlockTime};

/// Timing information for an account with regard to when its balance is accessable
/// This is to allow vesting from an initial genesis allocation
#[derive(Debug, Clone)]
pub enum Timing {
    /// Account does not have any timing limitations
    Untimed,
    /// Account does have timing limitations as specified
    Timed {
        /// Initial balance for the account
        initial_minimum_balance: Amount,
        /// Time when all balance is avaiable
        cliff_time: BlockTime,
        /// Amount extra available when fully fested
        cliff_amount: Amount,
        /// Ammount released in each vesting period
        vesting_increment: Amount,
        /// Period in whcih allocation is released in chunks
        vesting_period: BlockTime,
    },
}

impl Default for Timing {
    fn default() -> Self {
        Timing::Untimed
    }
}
