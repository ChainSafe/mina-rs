// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::numbers::{self, Length};

#[derive(Clone, Serialize, Deserialize)]
pub struct GlobalSlot {
    slot_number: numbers::GlobalSlot,
    slots_per_epoch: Length,
}
