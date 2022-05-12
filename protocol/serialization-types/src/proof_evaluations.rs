// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Proof evaluations used by the protocol state proof

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofEvaluations {
    pub l: FieldElementVecV1,
    pub r: FieldElementVecV1,
    pub o: FieldElementVecV1,
    pub z: FieldElementVecV1,
    pub t: FieldElementVecV1,
    pub f: FieldElementVecV1,
    pub sigma1: FieldElementVecV1,
    pub sigma2: FieldElementVecV1,
}

pub type ProofEvaluationsV1 = Versioned<ProofEvaluations, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofEvaluations)]
pub struct ProofEvaluationsJson {
    pub l: FieldElementVecJson,
    pub r: FieldElementVecJson,
    pub o: FieldElementVecJson,
    pub z: FieldElementVecJson,
    pub t: FieldElementVecJson,
    pub f: FieldElementVecJson,
    pub sigma1: FieldElementVecJson,
    pub sigma2: FieldElementVecJson,
}
