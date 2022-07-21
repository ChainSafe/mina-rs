// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account Timing

use ark_ff::{One, Zero};
use mina_serialization_types_macros::AutoFrom;
use once_cell::sync::OnceCell;
use proof_systems::mina_hasher::{Fp, Hashable, ROInput};
use smart_default::SmartDefault;

use crate::numbers::{Amount, BlockTime};

/// Payload for the timing variant Timed
#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::account::TimedData)]
#[auto_from(mina_serialization_types::account::TimedDataV0)]
pub struct TimedData {
    /// Initial balance for the account
    pub initial_minimum_balance: Amount,
    /// Time when all balance is avaiable
    pub cliff_time: BlockTime,
    /// Amount extra available when fully fested
    pub cliff_amount: Amount,
    /// Period in whcih allocation is released in chunks
    pub vesting_period: BlockTime,
    /// Ammount released in each vesting period
    pub vesting_increment: Amount,
}

impl Default for TimedData {
    fn default() -> Self {
        Self {
            initial_minimum_balance: 0.into(),
            cliff_time: 0.into(),
            cliff_amount: 0.into(),
            vesting_period: 1.into(),
            vesting_increment: 0.into(),
        }
    }
}

impl<'a> TimedData {
    /// Get a borrow of the default value
    pub fn borrow_default() -> &'a Self {
        static INSTANCE: OnceCell<TimedData> = OnceCell::new();
        INSTANCE.get_or_init(Self::default)
    }
}

impl Hashable for TimedData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.initial_minimum_balance)
            .append_hashable(&self.cliff_time)
            .append_hashable(&self.cliff_amount)
            .append_hashable(&self.vesting_period)
            .append_hashable(&self.vesting_increment);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

/// Timing information for an account with regard to when its balance is accessable
/// This is to allow vesting from an initial genesis allocation
#[derive(Debug, Clone, SmartDefault, AutoFrom)]
#[auto_from(mina_serialization_types::account::Timing)]
#[auto_from(mina_serialization_types::account::TimingV0)]
pub enum Timing {
    /// Account does not have any timing limitations
    #[default]
    Untimed,
    /// Account does have timing limitations as specified
    Timed(TimedData),
}

impl Hashable for Timing {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        match &self {
            Self::Untimed => {
                // FIXME: This is incorrect
                _ = Fp::zero();
                roi.append_field(Fp::one());
                // roi.append_field(Fp::zero());
                // roi.append_u32(1);
                // roi.append_hashable(TimedData::borrow_default());
            }
            Self::Timed(timed) => {
                roi.append_field(Fp::one());
                roi.append_u32(1);
                roi.append_hashable(timed);
            }
        }
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
