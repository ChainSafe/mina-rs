// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Versioned types that represent finite field and elliptic curve elements, and collections thereof

use crate::v1::BigInt256;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// Represents an element in a finite field that can be encoded as
/// a BigInt256. All finite field elements used in Mina satisfiy this requirement
pub type FieldElement = BigInt256;

pub type FieldElementVecV1 = Versioned<Vec<FieldElement>, 1>;

/// An elliptic curve point defined over a base field with elements that fit in a BigInt256
/// This is a Finite elliptic curve point as this type cannot be used to encode the point-at-infinity
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FiniteECPoint(pub FieldElement, pub FieldElement);

/// Vector of finite EC points (with version number)
pub type FiniteECPointVecV1 = Versioned<Vec<FiniteECPoint>, 1>;

pub type FiniteECPointPair = (FiniteECPoint, FiniteECPoint);

/// Vector of 2-tuples of finite EC points (with version number)
pub type FiniteECPointPairVecV1 = Versioned<Vec<FiniteECPointPair>, 1>;

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ECPoint {
    // elliptic curve point, can be the point at infinity
    Infinite,
    Finite(FiniteECPoint),
}

impl Default for ECPoint {
    fn default() -> Self {
        Self::Infinite
    }
}

pub type ECPointV1 = Versioned<ECPoint, 1>;

/// Vector of EC points (with version number defined in the WireType)
pub type ECPointVecV1 = Versioned<Vec<ECPointV1>, 1>;
