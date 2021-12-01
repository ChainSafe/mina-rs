// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use ark_ec::models::ModelParameters;
use ark_ec::short_weierstrass_jacobian::GroupAffine;
use num::Integer;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::numbers::BigInt256;

/// Represents an element in a finite field that can be encoded as
/// a BigInt256. All finite field elements used in Mina satisfiy this requirement
pub type FieldElement = BigInt256;

/// Vector of finite field elements (with version number defined in the WireType)
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FieldElementVec(pub Vec<FieldElement>);

impl FieldElementVec {
    pub fn to_hex_string(&self) -> String {
        let mut bytes = Vec::with_capacity(self.0.len() * 32);
        for i in &self.0 {
            for b in i.0 {
                bytes.push(b);
            }
        }
        hex::encode(bytes)
    }

    pub fn try_from_hex_string(s: impl AsRef<[u8]>) -> Result<Self, hex::FromHexError> {
        let mut s = s.as_ref();
        if s[1] == b'x' && (s[0] == b'0' || s[0] == b'\\') {
            s = &s[2..];
        }
        let bytes = hex::decode(s)?;
        let (q, r) = bytes.len().div_rem(&32);
        let mut vec = Vec::with_capacity(match r > 0 {
            true => q + 1,
            _ => q,
        });
        bytes.chunks(32).for_each(|chunk| {
            let mut b32 = [0; 32];
            b32.copy_from_slice(chunk);
            vec.push(BigInt256(b32));
        });

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_vec_roundtrip() {
        let hex_str = "16eba2ebda9feac442e29ef9293f5c4576933d531a6e3c07518e352241055f3d";
        let v = FieldElementVec::try_from_hex_string(hex_str).unwrap();
        assert_eq!(v.to_hex_string(), hex_str);

        let v = FieldElementVec::try_from_hex_string(format!("0x{}", hex_str)).unwrap();
        assert_eq!(v.to_hex_string(), hex_str);

        let v = FieldElementVec::try_from_hex_string(format!("\\x{}", hex_str)).unwrap();
        assert_eq!(v.to_hex_string(), hex_str);

        FieldElementVec::try_from_hex_string(format!("8x{}", hex_str)).expect_err("error expected");
    }

    #[test]
    fn test_field_element_vec_2_roundtrip() {
        let hex_strs = [
            "717115e59713c84f88babe2ec0292518060d2cc82b54e9a9c9a2d2a87ce91e15",
            "6994e270f284a557c418afebfaaca2794c8af6a476cb1b9478c205e8a901170f",
        ];
        let v = FieldElementVec::try_from_hex_string(hex_strs.join("")).unwrap();
        assert_eq!(v.0.len(), hex_strs.len());
        for i in 0..hex_strs.len() {
            assert_eq!(hex::encode(v.0[i].0), hex_strs[i]);
        }
    }
}
