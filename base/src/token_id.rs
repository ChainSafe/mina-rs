// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina blockchain can support multiple tokens, each with its own ID

use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
/// Newtype for TokenIds
pub struct TokenId(pub u64);
