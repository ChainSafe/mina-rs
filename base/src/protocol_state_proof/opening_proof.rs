// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_serialization_types_macros::AutoFrom;

use crate::types::{FieldElement, FiniteECPoint, FiniteECPointPairVec};

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::opening_proof::OpeningProof)]
pub struct OpeningProof {
    pub lr: FiniteECPointPairVec,
    pub z_1: FieldElement,
    pub z_2: FieldElement,
    pub delta: FiniteECPoint,
    pub sg: FiniteECPoint,
}
