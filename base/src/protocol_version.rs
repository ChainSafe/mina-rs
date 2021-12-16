// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Protocol version structure

use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
/// Defines a version of the Mina protocol in semver format
pub struct ProtocolVersion {
    /// Major version number
    major: u32,
    /// Minor version number
    minor: u32,
    /// Patch version number
    patch: u32,
}
