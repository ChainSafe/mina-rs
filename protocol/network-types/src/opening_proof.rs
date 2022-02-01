// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! The opening proof used by the protocol state proof

use crate::field_and_curve_elements::{FieldElement, FiniteECPoint, FiniteECPointPairVecV1};
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpeningProof {
    pub lr: FiniteECPointPairVecV1,
    pub z_1: FieldElement,
    pub z_2: FieldElement,
    pub delta: FiniteECPoint,
    pub sg: FiniteECPoint,
}

pub type OpeningProofV1 = Versioned<OpeningProof, 1>;
