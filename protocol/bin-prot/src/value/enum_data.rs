// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// This is the data that needs to be serialized and passed
/// to the serde deserializer so that the value::Sum can know
/// its variant name and index as described in the layout
#[derive(Serialize, Deserialize)]
pub struct EnumData {
    /// Index of variant
    pub index: u8,
    /// Name of variant
    pub name: String,
}
