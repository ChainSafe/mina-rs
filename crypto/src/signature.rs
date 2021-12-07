// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::{
    base58::{version_bytes, Base58Encodable},
    hash::BaseHash,
    impl_bs58_for_binprot,
};
use derive_deref::Deref;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

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

impl_bs58_for_binprot!(PublicKey, version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED);

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

impl Base58Encodable for Signature {
    const VERSION_BYTE: u8 = version_bytes::SIGNATURE;
    const MINA_VERSION_BYTE: u8 = 1;
    const MINA_VERSION_BYTE_COUNT: usize = 1;

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(64);
        let field_point_bytes: &[u8; 32] = self.0 .0 .0.as_ref();
        for &b in field_point_bytes {
            buf.push(b);
        }
        let inner_curve_scalar_bytes: &[u8; 32] = self.0 .1 .0.as_ref();
        for &b in inner_curve_scalar_bytes {
            buf.push(b);
        }
        buf
    }
}

impl From<Vec<u8>> for Signature {
    fn from(bytes: Vec<u8>) -> Self {
        // skip the bs58 version byte and mina bin_prot version byte
        let mut b32 = [0; 32];
        b32.copy_from_slice(&bytes[..32]);
        let field_point = FieldPoint(b32.into());
        b32.copy_from_slice(&bytes[32..]);
        let inner_curve_scalar = InnerCurveScalar(b32.into());
        Self((field_point, inner_curve_scalar))
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
    fn public_key_from_base58_roundtrip() {
        let s = "B62qonDZEKYULNkfq7WGu1Z881YBRnMSuBGGX5DhnTv26mUyvN99mpo";
        let k = PublicKey::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58_string())
    }

    #[test]
    fn signature_from_base58_roundtrip() {
        let s = "7mXTB1bcHYLJTmTfMtTboo4FSGStvera3z2wd6qjSxhpz1hZFMZZjcyaWAFEmZhgbq6DqVqGsNodnYKsCbMAq7D8yWo5bRSd";
        let k = Signature::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58_string())
    }
}
