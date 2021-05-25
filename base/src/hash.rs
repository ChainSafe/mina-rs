// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Hash and Hasher types reused throughout
//!

use serde::{Deserialize, Serialize};

pub use sha2::Sha256 as DefaultHasher;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct Hash(u64);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct StateHash(Hash);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct LedgerHash(Hash);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct EpochSeed(Hash);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct SnarkedLedgerHash(Hash);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct StagedLedgerHash(Hash);
