
use serde::{Serialize, Deserialize};

use crate::numbers::{self, Length};

#[derive(Serialize, Deserialize)]
pub struct GlobalSlot {
	slot_number: numbers::GlobalSlot,
	slots_per_epoch: Length
}
