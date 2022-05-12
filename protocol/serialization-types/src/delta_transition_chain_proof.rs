// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Delta transition chain proof structures and functions

use crate::common::*;
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};

/// Proof that the block was produced within the allotted slot time
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DeltaTransitionChainProof(pub HashV1, pub Vec<HashV1>);

/// Proof that the block was produced within the allotted slot time (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(DeltaTransitionChainProof)]
pub struct DeltaTransitionChainProofJson(pub LedgerHashV1Json, pub Vec<LedgerHashV1Json>);
