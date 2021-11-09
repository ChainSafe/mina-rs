// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use ark_ec::models::ModelParameters;
use ark_ec::short_weierstrass_jacobian::GroupAffine;

use crate::numbers::BigInt256;

/// Represents an element in a finite field that can be encoded as
/// a BigInt256. All finite field elements used in Mina satisfiy this requirement
pub type FieldElement = BigInt256;

/// Vector of finite field elements (with version number defined in the WireType)
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FieldElementVec(pub Vec<FieldElement>);

impl<Fs> Into<Vec<Fs>> for FieldElementVec
where
    Fs: From<ark_ff::BigInteger256>,
{
    fn into(self) -> Vec<Fs> {
        self.0
            .into_iter()
            .map(|i| ark_ff::BigInteger256::from(i).into())
            .collect()
    }
}

/// An elliptic curve point defined over a base field with elements that fit in a BigInt256
/// This is a Finite elliptic curve point as this type cannot be used to encode the point-at-infinity
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
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

/// Vector of finite EC points (with version number defined in the WireType)
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FiniteECPointVec(Vec<FiniteECPoint>);

impl<P> From<FiniteECPointVec> for Vec<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(v: FiniteECPointVec) -> Self {
        v.0.into_iter().map(Into::into).collect()
    }
}

/// Vector of 2-tuples of finite EC points (with version number defined in the WireType)
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FiniteECPointPairVec(Vec<(FiniteECPoint, FiniteECPoint)>);

impl<P> From<FiniteECPointPairVec> for Vec<(GroupAffine<P>, GroupAffine<P>)>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(v: FiniteECPointPairVec) -> Self {
        v.0.into_iter().map(|(x, y)| (x.into(), y.into())).collect()
    }
}

/// Elliptic curve point that can either be the coordinates of a point on the curve
/// OR it can be the point-at-infinity
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
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
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ECPointVec(Vec<ECPoint>);

impl<P> From<ECPointVec> for Vec<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(v: ECPointVec) -> Self {
        v.0.into_iter().map(Into::into).collect()
    }
}
