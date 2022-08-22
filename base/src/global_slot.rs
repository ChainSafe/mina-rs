// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Structure of a global slot

use crate::numbers::{self, Length};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    mina_hasher::{Hashable, ROInput},
    ChunkedROInput, ToChunkedROInput,
};

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::global_slot::GlobalSlot)]
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
        ROInput::new()
            .append_hashable(&self.slot_number)
            .append_hashable(&self.slots_per_epoch)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for GlobalSlot {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.slot_number)
            .append_chunked(&self.slots_per_epoch)
    }
}
