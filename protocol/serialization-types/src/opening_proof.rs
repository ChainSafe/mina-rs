// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! The opening proof used by the protocol state proof

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::{
    field_and_curve_elements::{FieldElement, FiniteECPoint},
    json::*,
    v1::*,
};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OpeningProof {
    pub lr: FiniteECPointPairVecV1,
    pub z_1: FieldElement,
    pub z_2: FieldElement,
    pub delta: FiniteECPoint,
    pub sg: FiniteECPoint,
}

pub type OpeningProofV1 = Versioned<OpeningProof, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, AutoFrom)]
#[auto_from(OpeningProof)]
pub struct OpeningProofJson {
    pub lr: FiniteECPointPairVecJson,
    pub z_1: FieldElementJson,
    pub z_2: FieldElementJson,
    pub delta: FiniteECPointJson,
    pub sg: FiniteECPointJson,
}
