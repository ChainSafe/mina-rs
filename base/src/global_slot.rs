// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::numbers::{self, Length};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct GlobalSlot {
    pub slot_number: numbers::GlobalSlot,
    pub slots_per_epoch: Length,
}
