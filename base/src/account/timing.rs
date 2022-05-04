// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account Timing

use mina_serialization_types_macros::AutoFrom;
use smart_default::SmartDefault;

use crate::numbers::{Amount, BlockTime};

/// Payload for the timing variant Timed
#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::account::TimedData)]
pub struct TimedData {
    /// Initial balance for the account
    pub initial_minimum_balance: Amount,
    /// Time when all balance is avaiable
    pub cliff_time: BlockTime,
    /// Amount extra available when fully fested
    pub cliff_amount: Amount,
    /// Ammount released in each vesting period
    pub vesting_increment: Amount,
    /// Period in whcih allocation is released in chunks
    pub vesting_period: BlockTime,
}

/// Timing information for an account with regard to when its balance is accessable
/// This is to allow vesting from an initial genesis allocation
#[derive(Debug, Clone, SmartDefault, AutoFrom)]
#[auto_from(mina_serialization_types::account::Timing)]
pub enum Timing {
    /// Account does not have any timing limitations
    #[default]
    Untimed,
    /// Account does have timing limitations as specified
    Timed(TimedData),
}
