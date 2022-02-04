// Copyright 2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and functions related to the Mina stake_delegation

use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use wire_type::WireType;
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]

/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/stake_delegation.ml#L23
pub struct StakeDelegation {
    /// Someone who delegates (= gives) part of their stake
    pub delegator: PublicKey,
    /// New delegating mina
    pub new_delegate: PublicKey,
}
