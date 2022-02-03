// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the Mina new_token_payload

use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]

/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/new_token_payload.ml#L8
pub struct NewTokenPayload {
    /// The public key of the token_owner
    pub token_owner_pk: PublicKey,
    /// Disable new account
    pub disable_new_accounts: bool,
}
