// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Proof evaluations used by the protocol state proof

use crate::field_and_curve_elements::FieldElementVecV1;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
