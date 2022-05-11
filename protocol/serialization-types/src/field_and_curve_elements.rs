// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Versioned types that represent finite field and elliptic curve elements, and collections thereof

use crate::{v1::*, *};
use derive_more::{From, Into};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smart_default::SmartDefault;
use versioned::Versioned;

/// Represents an element in a finite field that can be encoded as
/// a BigInt256. All finite field elements used in Mina satisfiy this requirement
pub type FieldElement = BigInt256;

/// Wrapper type for field element denoting it is on the curves scalar field
pub type InnerCurveScalar = BigInt256;

/// Vector of finite field elements
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FieldElementVec(pub Vec<FieldElement>);

/// Vector of finite field elements (v1)
pub type FieldElementVecV1 = Versioned<FieldElementVec, 1>;

/// Vector of finite field elements (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(FieldElementVec)]
pub struct FieldElementVecJson(pub Vec<FieldElementJson>);

/// An elliptic curve point defined over a base field with elements that fit in a BigInt256
/// This is a Finite elliptic curve point as this type cannot be used to encode the point-at-infinity
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiniteECPoint(pub FieldElement, pub FieldElement);

/// Vector of finite EC points
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiniteECPointVec(pub Vec<FiniteECPoint>);

/// Vector of finite EC points (with version number)
pub type FiniteECPointVecV1 = Versioned<FiniteECPointVec, 1>;

/// Vector of finite EC points (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(FiniteECPoint)]
pub struct FiniteECPointJson(pub FieldElementJson, pub FieldElementJson);

/// Pair if finite EC Points
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiniteECPointPair(pub FiniteECPoint, pub FiniteECPoint);

/// Pair if finite EC Points (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(FiniteECPointPair)]
pub struct FiniteECPointPairJson(pub FiniteECPointJson, pub FiniteECPointJson);

/// Vector of 2-tuples of finite EC points
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiniteECPointPairVec(pub Vec<FiniteECPointPair>);

/// Vector of 2-tuples of finite EC points (with version number)
pub type FiniteECPointPairVecV1 = Versioned<FiniteECPointPairVec, 1>;

/// Vector of 2-tuples of finite EC points (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(FiniteECPointPairVec)]
pub struct FiniteECPointPairVecJson(pub Vec<FiniteECPointPairJson>);

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SmartDefault)]
pub enum ECPoint {
    /// The point at infinity
    #[default]
    Infinite,
    /// Point on the curve
    Finite(FiniteECPoint),
}

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity (v1)
pub type ECPointV1 = Versioned<ECPoint, 1>;

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SmartDefault)]
enum ECPointJson {
    /// The point at infinity
    #[default]
    Infinite,
    /// Point on the curve
    Finite(FiniteECPointJson),
}

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity (mina json)
#[derive(Clone, Debug, PartialEq, SmartDefault, AutoFrom)]
#[auto_from(ECPoint)]
#[auto_from(ECPointJson)]
pub enum ECPointMinaJson {
    /// The point at infinity
    #[default]
    Infinite,
    /// Point on the curve
    Finite(FiniteECPointJson),
}

impl_mina_enum_json_serde!(ECPointMinaJson, ECPointJson);

/// Vector of EC points
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ECPointVec(pub Vec<ECPointV1>);

/// Vector of EC points (with version number defined in the WireType)
pub type ECPointVecV1 = Versioned<ECPointVec, 1>;

/// Vector of EC points (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ECPointVec)]
pub struct ECPointVecJson(pub Vec<ECPointMinaJson>);

/// Field element (json)
#[derive(Debug, Clone, PartialEq, From, Into)]
pub struct FieldElementJson(pub FieldElement);

impl FieldElementJson {
    /// Get hex string repr
    pub fn to_hex_string(&self) -> String {
        hex::encode(&self.0[..])
    }

    /// Convert from hex str
    pub fn try_from_hex_str(s: impl AsRef<[u8]>) -> Result<Self, hex::FromHexError> {
        let s = skip_0x_prefix_when_needed(s.as_ref());
        let bytes = hex::decode(s)?;
        let mut b32 = [0; 32];
        b32.copy_from_slice(&bytes);
        Ok(Self(b32))
    }
}

impl Serialize for FieldElementJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("0x{}", self.to_hex_string());
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for FieldElementJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from_hex_str(s).map_err(<D::Error as serde::de::Error>::custom)
    }
}

fn skip_0x_prefix_when_needed(s: &[u8]) -> &[u8] {
    if s[1] == b'x' && (s[0] == b'0' || s[0] == b'\\') {
        &s[2..]
    } else {
        s
    }
}
