// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Signatures and public key types

use crate::field_and_curve_elements::FieldElement;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// An EC point stored in compressed form containing only the x coordinate and one extra bit
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CompressedCurvePoint {
    /// The x coordinate of the EC point
    pub x: FieldElement,
    /// If the point is odd (or even)
    pub is_odd: bool,
}

/// Wrapper type for field element denoting it is on the curves scalar field
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InnerCurveScalar(pub FieldElement);

/// Public key (v1)
pub type PublicKeyV1 = Versioned<Versioned<CompressedCurvePoint, 1>, 1>;

/// Public key (v1) with an extra version byte
pub type PublicKey2V1 = Versioned<PublicKeyV1, 1>; // with an extra version wrapper

/// Signature (v1)
pub type SignatureV1 = Versioned<Versioned<(FieldElement, InnerCurveScalar), 1>, 1>;
