// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Structure of a global slot

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::numbers::{self, Length};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// A global slot
pub struct GlobalSlot {
    /// The global slot number of a chain or block
    pub slot_number: numbers::GlobalSlotNumber,
    /// Number of slots per epoch
    pub slots_per_epoch: Length,
}
