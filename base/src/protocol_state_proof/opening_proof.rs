// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::{FieldElement, FiniteECPoint, FiniteECPointPairVec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct OpeningProof {
    pub lr: FiniteECPointPairVec,
    pub z_1: FieldElement,
    pub z_2: FieldElement,
    pub delta: FiniteECPoint,
    pub sg: FiniteECPoint,
}
