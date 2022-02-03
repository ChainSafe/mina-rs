// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! types and functions related to Mina fee_transfer

use crate::numbers::Amount;
use crate::token_id::TokenId;
use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/fee_transfer.ml#L9
pub struct FeeTransfer {
    ///
    pub receiver_pk: PublicKey,
    ///
    pub fee: Amount,
    ///
    pub fee_token: TokenId,
}
