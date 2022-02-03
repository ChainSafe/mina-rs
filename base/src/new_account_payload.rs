// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the Mina new_account_payload

use crate::token_id::TokenId;
use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]

/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/new_account_payload.ml
pub struct NewAccountPayload {
    /// Token is implemented as ERC20 smart contract
    pub token_id: TokenId,
    /// The public key of the token_owner
    pub token_owner_pk: PublicKey,
    /// The public key of the intended recipient
    pub receiver_pk: PublicKey,
    /// The account has been disabled
    pub account_disabled: bool,
}
