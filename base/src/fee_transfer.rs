// Copyright 2022 ChainSafe Systems
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
    /// The public key of the intended recipient
    pub receiver_pk: PublicKey,
    /// The fee to be paid to the network to process the transaction
    pub fee: Amount,
    /// The token used by Mina Protocol to execute network transactions.
    pub fee_token: TokenId,
}
