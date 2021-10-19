// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::numbers::{self, Length};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct GlobalSlot {
    pub slot_number: numbers::GlobalSlotNumber,
    pub slots_per_epoch: Length,
}
