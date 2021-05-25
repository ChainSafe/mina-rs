// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Length(u32);

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Amount(u64);

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct GlobalSlot(u32);

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct BlockTime(u64);

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct BlockTimeSpan(u64);
