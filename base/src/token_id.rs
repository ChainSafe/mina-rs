// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina blockchain can support multiple tokens, each with its own ID

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash, Default)]
/// Newtype for TokenIds
pub struct TokenId(pub u64);
