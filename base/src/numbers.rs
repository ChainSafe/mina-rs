// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Newtypes for different numeric types used throughout Mina

use std::fmt;

use derive_deref::Deref;
use derive_more::From;
use mina_crypto::{hex::skip_0x_prefix_when_needed, prelude::*};
use num::Integer;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::Duration;

use crate::constants::MINA_PRECISION;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, From)]
#[from(forward)]
/// Newtype for TokenIds
pub struct TokenId(pub u64);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, Deref, From,
)]
#[from(forward)]

/// Represents the length of something (e.g. an epoch or window)
pub struct Length(pub u32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, From,
)]
#[from(forward)]

/// Represents a difference between two lengths
pub struct Delta(pub u32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, From,
)]
#[from(forward)]
// FIXME: 255 255 cannot be deserialized to u32, use i32 for now
// Note: Extended_Uint32 is not defined in bin_prot, but comes from mina
// Block path: t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/t/t/payload/t/t/common/t/t/t/valid_until
/// u32 wrapped in 1 version byte
/// This will not be part of the public API once the deserialization refactor is complete
pub struct ExtendedU32(pub i32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, From,
)]
#[from(forward)]

/// This will not be part of the public API once the deserialization refactor is complete
pub struct ExtendedU64(pub u64);

/// This structure represents fixed point numbers
/// typically amounts of Mina currency
/// # Example
/// ```
/// use mina_rs_base::numbers::*;
///
/// let amount = Amount(1000000030);
/// assert_eq!(amount.to_string(), "1.000000030");
/// ```
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, From)]
#[from(forward)]

pub struct Amount(pub u64);

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (q, r) = self.0.div_rem(&MINA_PRECISION);
        write!(f, "{}.{:#09}", q, r)
    }
}

#[derive(Debug, Error, PartialEq)]
/// Error that can be returned when parsing an Amount from string
pub enum ParseAmountError {
    /// Error occurs when parsing the integer components
    #[error("Error parsing integer in Amount")]
    ErrorParsingInteger(#[from] std::num::ParseIntError),

    /// Unable to split the string on a '.' into to integer parts
    #[error("Unexpected formatting, does not contain two integers seperated by a '.'. Got: {0}")]
    ErrorInvalidFormat(String),
}

impl std::str::FromStr for Amount {
    type Err = ParseAmountError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('.');
        let q: u64 = iter
            .next()
            .ok_or_else(|| Self::Err::ErrorInvalidFormat(s.to_string()))?
            .parse()?;
        let r: u64 = iter
            .next()
            .ok_or_else(|| Self::Err::ErrorInvalidFormat(s.to_string()))?
            .parse()?;
        if iter.next().is_none() {
            // ensure there isn't more to parse as that is undefined
            Ok(Amount(r + MINA_PRECISION * q))
        } else {
            Err(Self::Err::ErrorInvalidFormat(s.to_string()))
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, From)]
#[from(forward)]

/// 4 bytes wrapped by a version
/// Will not form part of the public API when deserialization refactor is complete
pub struct Hex64(pub i64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, From)]
#[from(forward)]

/// A single char defined by a single byte (not variable length like a Rust char)
pub struct Char(pub u8);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, Deref, From)]
#[from(forward)]
/// A global slot number
pub struct GlobalSlotNumber(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, From)]
/// Block time numeric type
pub struct BlockTime(pub u64);

impl BlockTime {
    /// Unix timestamp conversion (seconds since the unix epoch)
    pub fn from_unix_epoch(ts: u64) -> Self {
        Self::from_unix_epoch_millis(ts * 1000)
    }

    /// Unix timestamp conversion (milliseconds since the unix epoch)
    pub fn from_unix_epoch_millis(ts: u64) -> Self {
        Self(ts)
    }

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

impl From<mina_serialization_types::v1::BlockTimeV1> for BlockTime {
    fn from(t: mina_serialization_types::v1::BlockTimeV1) -> Self {
        Self(t.t.t)
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, From)]
#[from(forward)]
/// Time span between two block time instants
pub struct BlockTimeSpan(pub u64);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, From)]
#[from(forward)]
/// Mina 256 bit Bit Integer type
pub struct BigInt256(pub [u8; 32]);

impl AsRef<[u8]> for BigInt256 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl HexEncodable for BigInt256 {
    type Error = hex::FromHexError;

    fn to_hex_string(&self) -> String
    where
        Self: AsRef<[u8]>,
    {
        hex::encode(self)
    }

    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error> {
        let s = skip_0x_prefix_when_needed(s.as_ref());
        let bytes = hex::decode(s)?;
        let mut b32 = [0; 32];
        b32.copy_from_slice(&bytes);
        Ok(Self(b32))
    }
}

impl From<BigInt256> for ark_ff::BigInteger256 {
    fn from(i: BigInt256) -> Self {
        use ark_ff::bytes::FromBytes;
        Self::read(&i.0[..]).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::numbers::*;
    use crate::types::ParseAmountError;
    use std::str::FromStr;

    #[test]
    pub fn test_amount_to_string() {
        assert_eq!(Amount(0).to_string(), "0.000000000");
        assert_eq!(Amount(3).to_string(), "0.000000003");
        assert_eq!(Amount(1000000003).to_string(), "1.000000003");
        assert_eq!(Amount(1000000030).to_string(), "1.000000030");
        assert_eq!(Amount(1300000000).to_string(), "1.300000000");
        assert_eq!(Amount(1000000000).to_string(), "1.000000000");
    }

    #[test]
    pub fn test_amount_from_string() {
        assert_eq!(Amount::from_str("0.000000000").unwrap(), Amount(0));
        assert_eq!(Amount::from_str("0.000000003").unwrap(), Amount(3));
        assert_eq!(Amount::from_str("1.000000003").unwrap(), Amount(1000000003));
        assert_eq!(Amount::from_str("1.000000030").unwrap(), Amount(1000000030));
        assert_eq!(Amount::from_str("1.300000000").unwrap(), Amount(1300000000));
        assert_eq!(Amount::from_str("1.000000000").unwrap(), Amount(1000000000));

        assert_eq!(
            Amount::from_str("0.000000000.0").unwrap_err(),
            ParseAmountError::ErrorInvalidFormat("0.000000000.0".to_string())
        );
        assert_eq!(
            Amount::from_str("000000000").unwrap_err(),
            ParseAmountError::ErrorInvalidFormat("000000000".to_string())
        );
    }
}
