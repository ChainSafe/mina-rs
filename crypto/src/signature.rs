// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::{base58::Base58Encodable, hash::BaseHash};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{BigInteger, BigInteger256, FromBytes};
use derive_deref::Deref;
use derive_more::From;
use mina_curves::pasta::{pallas, Fq};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use wire_type::WireType;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType, From)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PrivateKey([u8; 32]);

impl Base58Encodable for PrivateKey {
    const VERSION_BYTE: u8 = crate::base58::version_bytes::PRIVATE_KEY;
}

impl PrivateKey {
    pub fn derive_public_key(&self) -> Result<PublicKey, Error> {
        let scalar: Fq = self.try_to_fq()?;
        let prime = pallas::Affine::prime_subgroup_generator();
        let public_key_projective = prime.mul(scalar);
        let public_key_affine = public_key_projective.into_affine();
        let x = public_key_affine.x;
        let y = public_key_affine.y;
        let x_big: BigInteger256 = x.into();
        let y_big: BigInteger256 = y.into();
        let x_bytes_vec = x_big.to_bytes_le();
        let mut x_bytes = [0; 32];
        x_bytes.copy_from_slice(x_bytes_vec.as_slice());
        Ok(PublicKey {
            poly: CompressedCurvePoint {
                x: x_bytes,
                is_odd: y_big.get_bit(0),
            },
        })
    }

    pub fn validate(&self, public_key: &PublicKey) -> Result<bool, Error> {
        Ok(self.derive_public_key()?.borrow() == public_key)
    }

    pub fn try_to_bigint(&self) -> Result<BigInteger256, Error> {
        BigInteger256::read(self.0.as_slice()).map_err(Error::IoError)
    }

    pub fn try_to_fq(&self) -> Result<Fq, Error> {
        let i: BigInteger256 = self.try_to_bigint()?;
        Ok(i.into())
    }
}

impl TryInto<BigInteger256> for PrivateKey {
    type Error = Error;
    fn try_into(self) -> Result<BigInteger256, Self::Error> {
        self.try_to_bigint()
    }
}

impl TryInto<Fq> for PrivateKey {
    type Error = Error;
    fn try_into(self) -> Result<Fq, Self::Error> {
        self.try_to_fq()
    }
}

impl From<BigInteger256> for PrivateKey {
    fn from(i: BigInteger256) -> Self {
        let v = i.to_bytes_le();
        let mut bytes = [0; 32];
        bytes.copy_from_slice(v.as_slice());
        Self(bytes)
    }
}

impl From<Fq> for PrivateKey {
    fn from(scalar: Fq) -> Self {
        let i: BigInteger256 = scalar.into();
        i.into()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct CompressedCurvePoint {
    pub x: [u8; 32],
    pub is_odd: bool,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PublicKey {
    pub poly: CompressedCurvePoint,
}

impl Base58Encodable for PublicKey {
    const VERSION_BYTE: u8 = crate::base58::version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED;
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Deref, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PublicKey2(pub CompressedCurvePoint);

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Deref, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct PublicKey3(pub CompressedCurvePoint);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct Signature((FieldPoint, InnerCurveScalar));

impl Signature {
    /// field_point
    pub fn field_point(&self) -> &FieldPoint {
        &self.0 .0
    }

    /// inner_curve_scalar
    pub fn inner_curve_scalar(&self) -> &InnerCurveScalar {
        &self.0 .1
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct FieldPoint(BaseHash);

impl AsRef<[u8]> for FieldPoint {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct InnerCurveScalar(BaseHash);

impl AsRef<[u8]> for InnerCurveScalar {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use bin_prot::to_writer;

    #[test]
    fn serialize_empty_keypair() {
        let mut buf = Vec::new();
        to_writer(&mut buf, &PublicKey::default()).unwrap();
        println!("{:?}", buf)
    }

    #[test]
    fn from_base58_roundtrip() {
        let s = "B62qonDZEKYULNkfq7WGu1Z881YBRnMSuBGGX5DhnTv26mUyvN99mpo";
        let k = PublicKey::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58().into_string())
    }

    #[test]
    fn validate_keypair() {
        // test key pairs can be generated from either mina generate-keypair tool or mina js client_sdk
        const PRIVATE_KEY: &str = "EKFNQSkz7Nw1iLpQyLYSMmzMqsMc9pve2R9bLpKnsJ2gsCjY7VLs";
        const PUBLIC_KEY: &str = "B62qnJsxEpjQnnWqW621sksXsAip8Mfz1MB7gmFsuVxamRuvSaAeLxx";
        let private_key = PrivateKey::from_base58(PRIVATE_KEY).unwrap();
        let public_key = PublicKey::from_base58(PUBLIC_KEY).unwrap();
        assert!(private_key.validate(&public_key).unwrap());
    }
}
