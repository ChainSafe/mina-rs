// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use ark_ec::models::ModelParameters;
use ark_ec::short_weierstrass_jacobian::GroupAffine;
use mina_crypto::{hex::skip_0x_prefix_when_needed, prelude::*};
use mina_serialization_types_macros::AutoFrom;
use num::Integer;

use crate::numbers::BigInt256;

/// Represents an element in a finite field that can be encoded as
/// a BigInt256. All finite field elements used in Mina satisfiy this requirement
pub type FieldElement = BigInt256;

/// Vector of finite field elements (with version number defined in the WireType)
#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::FieldElementVec)]
pub struct FieldElementVec(pub Vec<FieldElement>);

impl HexEncodable for FieldElementVec {
    type Error = hex::FromHexError;

    fn to_hex_string(&self) -> String {
        let mut s = String::with_capacity(64 * self.0.len());
        for i in &self.0 {
            s.push_str(&i.to_hex_string());
        }
        s
    }

    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error> {
        let s = skip_0x_prefix_when_needed(s.as_ref());
        let (q, r) = s.len().div_rem(&64);
        let mut vec = Vec::with_capacity(match r > 0 {
            true => q + 1,
            _ => q,
        });
        for chunk in s.chunks(64) {
            vec.push(BigInt256::try_from_hex(chunk)?);
        }

        Ok(Self(vec))
    }
}

impl<Fs> From<FieldElementVec> for Vec<Fs>
where
    Fs: From<ark_ff::BigInteger256>,
{
    fn from(t: FieldElementVec) -> Self {
        t.0.into_iter()
            .map(|i| ark_ff::BigInteger256::from(i).into())
            .collect()
    }
}

/// An elliptic curve point defined over a base field with elements that fit in a BigInt256
/// This is a Finite elliptic curve point as this type cannot be used to encode the point-at-infinity
#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::FiniteECPoint)]
pub struct FiniteECPoint(pub FieldElement, pub FieldElement);

impl<P> From<FiniteECPoint> for GroupAffine<P>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(FiniteECPoint(x, y): FiniteECPoint) -> Self {
        Self::new(
            ark_ff::BigInteger256::from(x).into(),
            ark_ff::BigInteger256::from(y).into(),
            false,
        )
    }
}

#[macro_export]
macro_rules! finite_ec_point {
    ($e1:expr, $e2:expr) => {
        (|s1, s2| {
            Ok::<_, hex::FromHexError>(FiniteECPoint(
                FieldElement::try_from_hex(s1)?,
                FieldElement::try_from_hex(s2)?,
            ))
        })($e1, $e2)
    };
}

/// Vector of finite EC points (with version number defined in the WireType)
#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::FiniteECPointVec)]
pub struct FiniteECPointVec(pub Vec<FiniteECPoint>);

impl<P> From<FiniteECPointVec> for Vec<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(v: FiniteECPointVec) -> Self {
        v.0.into_iter().map(Into::into).collect()
    }
}

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::FiniteECPointPair)]
pub struct FiniteECPointPair(pub FiniteECPoint, pub FiniteECPoint);

#[macro_export]
macro_rules! finite_ec_point_pair {
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr) => {
        (|s1, s2, s3, s4| {
            use mina_rs_base::finite_ec_point;
            use mina_rs_base::protocol_state_proof::*;
            Ok::<_, hex::FromHexError>(FiniteECPointPair(
                finite_ec_point!(s1, s2)?,
                finite_ec_point!(s3, s4)?,
            ))
        })($e1, $e2, $e3, $e4)
    };
}

/// Vector of 2-tuples of finite EC points (with version number defined in the WireType)
#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::FiniteECPointPairVec)]
pub struct FiniteECPointPairVec(pub Vec<FiniteECPointPair>);

impl<P> From<FiniteECPointPairVec> for Vec<(GroupAffine<P>, GroupAffine<P>)>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(v: FiniteECPointPairVec) -> Self {
        v.0.into_iter()
            .map(|FiniteECPointPair(x, y)| (x.into(), y.into()))
            .collect()
    }
}

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity
#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::ECPoint)]
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

impl<P> From<ECPoint> for GroupAffine<P>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(p: ECPoint) -> Self {
        match p {
            ECPoint::Infinite => Self::new(Default::default(), Default::default(), true),
            ECPoint::Finite(FiniteECPoint(x, y)) => Self::new(
                ark_ff::BigInteger256::from(x).into(),
                ark_ff::BigInteger256::from(y).into(),
                false,
            ),
        }
    }
}

/// Vector of EC points (with version number defined in the WireType)
#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::field_and_curve_elements::ECPointVec)]
pub struct ECPointVec(pub Vec<ECPoint>);

impl<P> From<ECPointVec> for Vec<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(v: ECPointVec) -> Self {
        v.0.into_iter().map(Into::into).collect()
    }
}
