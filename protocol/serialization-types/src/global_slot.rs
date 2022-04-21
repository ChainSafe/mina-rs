// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Structure of a global slot

use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::v1::{GlobalSlotNumberV1, LengthV1};

/// A global slot
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalSlot {
    /// The global slot number of a chain or block
    pub slot_number: GlobalSlotNumberV1,
    /// Number of slots per epoch
    pub slots_per_epoch: LengthV1,
}

/// A global slot (v1)
pub type GlobalSlotV1 = Versioned<Versioned<GlobalSlot, 1>, 1>;
