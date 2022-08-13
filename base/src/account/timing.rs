// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account Timing

use ark_ff::{One, Zero};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{mina_hasher::Fp, ChunkedROInput, ToChunkedROInput};
use smart_default::SmartDefault;

use crate::numbers::{Amount, BlockTime};

/// Payload for the timing variant Timed
#[derive(Clone, Debug, Eq, PartialEq, AutoFrom)]
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
    /// Amount released in each vesting period
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

impl ToChunkedROInput for TimedData {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.initial_minimum_balance)
            .append_chunked(&self.cliff_time)
            .append_chunked(&self.cliff_amount)
            .append_chunked(&self.vesting_period)
            .append_chunked(&self.vesting_increment)
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

impl ToChunkedROInput for Timing {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        match &self {
            Self::Untimed => ChunkedROInput::new()
                .append_packed(Fp::zero(), 1)
                .append_chunked(&TimedData::default()),
            Self::Timed(timed) => ChunkedROInput::new()
                .append_packed(Fp::one(), 1)
                .append_chunked(timed),
        }
    }
}
