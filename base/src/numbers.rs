// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use derive_deref::Deref;
use num::Integer;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

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
pub struct Length(pub u32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
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
pub struct ExtendedU32(pub i32);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
pub struct ExtendedU64_3(pub u64);

#[derive(
    Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default, WireType,
)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ExtendedU64_2(pub u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct Amount(pub u64);

impl Amount {
    // https://github.com/MinaProtocol/mina/pull/4306
    // https://github.com/MinaProtocol/mina/blob/ec00ece4606244e842bf90d989d6f9bb66ab275f/src/lib/currency/currency.ml#L68
    pub fn to_formatted_string(&self) -> String {
        const PRECISION: u32 = 9;
        const PRECISION_EXP: u64 = 10_u64.pow(PRECISION);
        let (q, r) = self.0.div_rem(&PRECISION_EXP);
        format!("{}.{}", q, Self::pad_to_width(r, PRECISION))
    }

    fn pad_to_width(r: u64, width: u32) -> String {
        let mut s = r.to_string();
        let num_zeros_to_pad = width - s.len() as u32;
        for _i in 0..num_zeros_to_pad {
            s.insert(0, '0');
        }
        s
    }
}

// TODO: Impl From<String> for Amount {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct Hex64(i64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct Char(u8);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, Deref, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct GlobalSlotNumber(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct BlockTime(u64);

impl BlockTime {
    pub fn epoch_millis(&self) -> u64 {
        self.0
    }

    pub fn datetime(&self) -> chrono::DateTime<chrono::Utc> {
        use chrono::prelude::*;
        Utc.timestamp_millis(self.0 as i64)
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
pub struct BlockTimeSpan(pub u64);

// Consider switch to [ark-ff](https://docs.rs/ark-ff/0.3.0/ark_ff/biginteger/struct.BigInteger256.html)
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct BigInt256(pub [u8; 32]);

#[cfg(test)]
pub mod tests {
    use crate::numbers::Amount;

    #[test]
    pub fn test_amount_to_formatted_string() {
        assert_eq!(Amount(0).to_formatted_string(), "0.000000000");
        assert_eq!(Amount(3).to_formatted_string(), "0.000000003");
        assert_eq!(Amount(1000000003).to_formatted_string(), "1.000000003");
        assert_eq!(Amount(1000000030).to_formatted_string(), "1.000000030");
        assert_eq!(Amount(1300000000).to_formatted_string(), "1.300000000");
        assert_eq!(Amount(1000000000).to_formatted_string(), "1.000000000");
    }
}
