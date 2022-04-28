// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Protocol version structure

use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// Defines a version of the Mina protocol in semver format
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolVersion {
    /// Major version number
    pub major: u32,
    /// Minor version number
    pub minor: u32,
    /// Patch version number
    pub patch: u32,
}

/// Defines a version of the Mina protocol in semver format (v1)
pub type ProtocolVersionV1 = Versioned<ProtocolVersion, 1>;

/// ProtocolVersion that is convertible from / to the mina specific json representation
pub type ProtocolVersionJson = ProtocolVersion;

impl From<ProtocolVersionV1> for ProtocolVersionJson {
    fn from(t: ProtocolVersionV1) -> Self {
        t.t
    }
}
