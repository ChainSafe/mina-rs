// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Definitions of some signature types
//! These are currently only used for serialization tests and will
//! be replaced by those in the 01-labs/proof-systems repo in the future

use crate::{
    base58::{version_bytes, Base58Encodable},
};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Signature(pub (FieldPoint, InnerCurveScalar));

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

    fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
        let field_point_bytes: &[u8; 32] = &self.0 .0 .0;
        output.extend(field_point_bytes);
        let inner_curve_scalar_bytes: &[u8; 32] = &self.0 .1 .0;
        output.extend(inner_curve_scalar_bytes);
    }
}

impl From<Vec<u8>> for Signature {
    fn from(bytes: Vec<u8>) -> Self {
        // skip the bs58 version byte and mina bin_prot version byte
        let mut b32 = [0; 32];
        b32.copy_from_slice(&bytes[..32]);
        let field_point = FieldPoint(b32);
        b32.copy_from_slice(&bytes[32..]);
        let inner_curve_scalar = InnerCurveScalar(b32);
        Self((field_point, inner_curve_scalar))
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, From, Into)]
#[into(owned, ref)]
pub struct FieldPoint([u8; 32]);

impl AsRef<[u8]> for FieldPoint {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, From, Into)]
#[into(owned, ref)]
pub struct InnerCurveScalar(pub [u8; 32]);

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
