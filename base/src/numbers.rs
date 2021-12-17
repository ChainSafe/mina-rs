// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Newtypes for different numeric types used throughout Mina

use std::fmt;

use derive_deref::Deref;
use num::Integer;
use serde::{Deserialize, Serialize};
use time::Duration;
use wire_type::WireType;

use crate::constants::MINA_PRECISION;

#[derive(
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    Debug,
    Hash,
    Copy,
    Default,
    Deref,
    WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// Represents the length of something (e.g. an epoch or window)
pub struct Length(pub u32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// Represents a difference between two lengths
pub struct Delta(pub u32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
// FIXME: 255 255 cannot be deserialized to u32, use i32 for now
// Note: Extended_Uint32 is not defined in bin_prot, but comes from mina
// Block path: t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/t/t/payload/t/t/common/t/t/t/valid_until
/// u32 wrapped in 1 version byte
/// This will not be part of the public API once the deserialization refactor is complete
pub struct ExtendedU32(pub i32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
/// u64 wrapped in 3 version bytes
/// This will not be part of the public API once the deserialization refactor is complete
pub struct ExtendedU64_3(pub u64);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// u64 wrapped in 2 version bytes
/// This will not be part of the public API once the deserialization refactor is complete
pub struct ExtendedU64_2(pub u64);

/// This structure represents fixed point numbers
/// typically amounts of Mina currency
/// # Example
/// ```
/// use mina_rs_base::numbers::*;
///
/// let amount = Amount(1000000030);
/// assert_eq!(amount.to_string(), "1.000000030");
/// ```
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct Amount(pub u64);

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (q, r) = self.0.div_rem(&MINA_PRECISION);
        write!(f, "{}.{:#09}", q, r)
    }
}

// TODO: Impl From<String> for Amount {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
/// 4 bytes wrapped by a version
/// Will not form part of the public API when deserialization refactor is complete
pub struct Hex64(i64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
/// A single char defined by a single byte (not variable length like a Rust char)
pub struct Char(u8);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, Deref, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// A global slot number
pub struct GlobalSlotNumber(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// Block time numeric type
pub struct BlockTime(u64);

impl BlockTime {
    /// Gets unix timestamp in milliseconds
    pub fn epoch_millis(&self) -> u64 {
        self.0
    }

    /// Gets timestamp in [time::OffsetDateTime] format
    pub fn datetime(&self) -> time::OffsetDateTime {
        use time::OffsetDateTime;
        let (q, r) = (self.0 as i64).div_rem(&1000);
        let dt = OffsetDateTime::from_unix_timestamp(q).expect("Invalid block time");
        if r == 0 {
            dt
        } else {
            dt + Duration::milliseconds(r)
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
/// Time span between two block time instants
pub struct BlockTimeSpan(pub u64);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
/// Mina 256 bit Bit Integer type
pub struct BigInt256(pub [u8; 32]);

impl From<BigInt256> for ark_ff::BigInteger256 {
    fn from(i: BigInt256) -> Self {
        use ark_ff::bytes::FromBytes;
        Self::read(&i.0[..]).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::numbers::Amount;
    use crate::numbers::BigInt256;

    #[test]
    pub fn test_amount_to_formatted_string() {
        assert_eq!(Amount(0).to_string(), "0.000000000");
        assert_eq!(Amount(3).to_string(), "0.000000003");
        assert_eq!(Amount(1000000003).to_string(), "1.000000003");
        assert_eq!(Amount(1000000030).to_string(), "1.000000030");
        assert_eq!(Amount(1300000000).to_string(), "1.300000000");
        assert_eq!(Amount(1000000000).to_string(), "1.000000000");
    }

    #[test]
    fn test_convert_bigint_to_arkworks_zero() {
        use ark_ff::BigInteger256;
        let i = BigInt256([0; 32]);
        let ark_i: BigInteger256 = i.into();
        assert_eq!(ark_i, BigInteger256::default())
    }
}
