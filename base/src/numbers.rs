
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Length(u32);

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Amount(u64);

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct GlobalSlot(u32);
