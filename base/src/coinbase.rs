// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina coinbase

use crate::numbers::ExtendedU64_2;
use mina_crypto::signature::PublicKey2;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/staged_ledger_diff/staged_ledger_diff.ml#L10
pub enum CoinBase {
    ///
    Zero,
    ///
    One(Option<CoinBaseFeeTransfer>),
    ///
    Two(Option<Option<CoinBaseFeeTransfer>>),
}

impl Default for CoinBase {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
// FIXME: No test coverage yet
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/coinbase.ml#L3
pub struct CoinBaseFeeTransfer {
    ///
    pub receiver_pk: PublicKey2,
    ///
    pub fee: ExtendedU64_2,
}
