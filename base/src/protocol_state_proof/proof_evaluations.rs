// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::FieldElementVec;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct ProofEvaluations {
    pub l: FieldElementVec,
    pub r: FieldElementVec,
    pub o: FieldElementVec,
    pub z: FieldElementVec,
    pub t: FieldElementVec,
    pub f: FieldElementVec,
    pub sigma1: FieldElementVec,
    pub sigma2: FieldElementVec,
}
