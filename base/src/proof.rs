// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Serialize, Deserialize};
use serde_versions_derive::version;

const PROOF_BYTE_LENGTH: usize = 32;

/// Placeholder type for a pickles proof
/// This will simply hold the bytes during deserialization
/// until this is ready to be implemented
#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Proof([u8; PROOF_BYTE_LENGTH]);
