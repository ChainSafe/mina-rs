// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Protocol version structure

#[derive(Clone, Debug, PartialEq)]
/// Defines a version of the Mina protocol in semver format
pub struct ProtocolVersion {
    /// Major version number
    pub major: u32,
    /// Minor version number
    pub minor: u32,
    /// Patch version number
    pub patch: u32,
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        ProtocolVersion {
            major: 2,
            minor: 0,
            patch: 0,
        }
    }
}
