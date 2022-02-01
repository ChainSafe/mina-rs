// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::field_and_curve_elements::FieldElement;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CompressedCurvePoint {
    pub x: [u8; 32],
    pub is_odd: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InnerCurveScalar(FieldElement);

pub type PublicKeyV1 = Versioned<Versioned<CompressedCurvePoint, 1>, 1>;

pub type PublicKey2V1 = Versioned<PublicKeyV1, 1>; // with an extra version wrapper

pub type SignatureV1 = Versioned<Versioned<(FieldElement, InnerCurveScalar), 1>, 1>;
