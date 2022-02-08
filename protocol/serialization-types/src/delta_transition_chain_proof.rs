// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Delta transition chain proof structures and functions

use crate::v1::HashV1;

/// Proof that the block was produced within the allotted slot time
pub type DeltaTransitionChainProof = (HashV1, Vec<HashV1>);
