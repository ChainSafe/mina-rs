// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types and funcions related to the Mina stake_delegation

use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};
use wire_type::WireType;
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]

///
pub struct StakeDelegation {
    ///
    pub delegator: PublicKey,
    ///
    pub new_delegate: PublicKey,
}
