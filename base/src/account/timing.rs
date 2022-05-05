// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account Timing

use mina_serialization_types_macros::AutoFrom;

use proof_systems::mina_hasher::{Hashable, ROInput};

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
#[derive(Debug, Clone, AutoFrom)]
#[auto_from(mina_serialization_types::account::Timing)]
pub enum Timing {
    /// Account does not have any timing limitations
    Untimed,
    /// Account does have timing limitations as specified
    Timed(TimedData),
}

// Auxiliary data structure for ROInput
#[derive(Default)]
struct TimingRecord {
    /// Whether timed
    is_timed: bool,
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
}

impl From<&Timing> for TimingRecord {
    fn from(timing: &Timing) -> Self {
        match timing {
            Timing::Untimed => {
                let mut timing_rec: TimingRecord = Default::default();
                timing_rec.vesting_period = BlockTime(1);
                timing_rec.is_timed = false;
                timing_rec
            }
            Timing::Timed(TimedData {
                initial_minimum_balance,
                cliff_time,
                cliff_amount,
                vesting_period,
                vesting_increment,
            }) => TimingRecord {
                is_timed: true,
                initial_minimum_balance: *initial_minimum_balance,
                cliff_time: cliff_time.clone(),
                cliff_amount: *cliff_amount,
                vesting_period: vesting_period.clone(),
                vesting_increment: *vesting_increment,
            },
        }
    }
}

impl Hashable for Timing {
    type D = ();

    fn to_roinput(&self) -> proof_systems::mina_hasher::ROInput {
        let mut roi = ROInput::new();
        let timing: TimingRecord = self.into();
        roi.append_bool(timing.is_timed);
        roi.append_hashable(&timing.initial_minimum_balance)
            .append_hashable(&timing.cliff_time)
            .append_hashable(&timing.cliff_amount)
            .append_hashable(&timing.vesting_increment)
            .append_hashable(&timing.vesting_period);

        roi
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        Some("CodaTiming".to_string())
    }
}

impl Default for Timing {
    fn default() -> Self {
        Timing::Untimed
    }
}
