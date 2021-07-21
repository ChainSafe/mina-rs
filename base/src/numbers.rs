// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Hash, Copy, Default)]
pub struct Length(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
pub struct Amount(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
pub struct GlobalSlot(pub u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
pub struct BlockTime(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
pub struct BlockTimeSpan(u64);
