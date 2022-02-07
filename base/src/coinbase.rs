// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the Mina coinbase

use crate::fee_transfer::FeeTransfer;
use crate::numbers::{Amount, ExtendedU64_2};
use mina_crypto::signature::{PublicKey, PublicKey2};
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/staged_ledger_diff/staged_ledger_diff.ml#L10
/// https://github.com/MinaProtocol/mina/blob/1c189f630365e2ee8f6f3f221bb11f9718efe024/src/lib/staged_ledger/diff_creation_log.ml#L44
/// Coin minting exchange
pub enum CoinBase {
    /// Match coinbase with zero
    Zero,
    /// Match coinbase with one
    One(Option<CoinBaseFeeTransfer>),
    /// Match coinbase with two
    Two(Option<CoinBaseFeeTransfer>, Option<CoinBaseFeeTransfer>),
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
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/coinbase.ml#L9
pub struct CoinBaseV1 {
    /// The public key of the intended recipient
    pub receiver: PublicKey,
    /// The amount of mina you are sending
    pub amount: Amount,
    /// Fee transfer single
    pub fee_transfer: FeeTransfer,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
// FIXME: No test coverage yet
///https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/coinbase_fee_transfer.ml#L8
pub struct CoinBaseFeeTransfer {
    /// The public key of the intended recipient
    pub receiver_pk: PublicKey2,
    /// The fee to be paid to the network to process the transaction
    pub fee: ExtendedU64_2,
}
