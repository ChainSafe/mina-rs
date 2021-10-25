// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::{
    base58::Base58Encodable,
    hash::{BackendCommonHash, BaseHash},
};
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct CompressedCurvePoint {
    x: BackendCommonHash,
    is_odd: bool,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PublicKey {
    poly: CompressedCurvePoint,
}

impl Base58Encodable for PublicKey {
    const VERSION_BYTE: u8 = crate::base58::version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED;
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct Signature((FieldPoint, InnerCurveScalar));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct FieldPoint(BaseHash);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct InnerCurveScalar(BaseHash);

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
}
