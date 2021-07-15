// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy)]
pub struct Length(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Amount(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct GlobalSlot(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct BlockTime(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct BlockTimeSpan(u64);
