// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Delta transition chain proof structures and functions

use crate::v1::HashV1;
use serde::{Deserialize, Serialize};

/// Proof that the block was produced within the allotted slot time
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeltaTransitionChainProof(pub HashV1, pub Vec<HashV1>);
