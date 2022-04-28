// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Structure of a global slot

use crate::numbers::{self, Length};
use proof_systems::mina_hasher::{Hashable, ROInput};

#[derive(Clone, Default, PartialEq, Debug)]
/// A global slot
pub struct GlobalSlot {
    /// The global slot number of a chain or block
    pub slot_number: numbers::GlobalSlotNumber,
    /// Number of slots per epoch
    pub slots_per_epoch: Length,
}

impl Hashable for GlobalSlot {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.slot_number);
        roi.append_hashable(&self.slots_per_epoch);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
