// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina minting_payload

use crate::numbers::Amount;
use crate::token_id::TokenId;
use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]

///
pub struct MintingPayload {
    pub token_id: TokenId,
    pub token_owner_pk: PublicKey,
    pub receiver_pk: PublicKey,
    pub amount: Amount,
}
