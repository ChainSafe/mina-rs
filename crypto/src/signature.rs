// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Definitions of some signature types
//! These are currently only used for serialization tests and will
//! be replaced by those in the 01-labs/proof-systems repo in the future

use crate::{
    base58::{version_bytes, Base58Encodable},
    hash::{BaseHash, RandomOraclePartialInput},
    impl_bs58_for_binprot,
};
use ark_ff::BigInteger256;
use derive_deref::Deref;
use mina_curves::pasta::Fp;
use mina_signer::ROInput;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct CompressedCurvePoint {
    pub x: [u8; 32],
    pub is_odd: bool,
}

impl CompressedCurvePoint {
    /// Const function that converts CompressedCurvePoint into BigInteger256
    pub const fn to_big256(&self) -> BigInteger256 {
        let mut array = [0_u64; 4];
        let mut buffer = [0_u8; 8];
        let mut i = 0;
        while i < 4 {
            let mut j = 0;
            while j < 8 {
                let offset = i * 8;
                buffer[j] = self.x[offset + j];
                j += 1;
            }
            array[i] = u64::from_le_bytes(buffer);
            i += 1;
        }
        BigInteger256::new(array)
    }
}

impl RandomOraclePartialInput<CompressedCurvePoint> for ROInput {
    /// This is equivilent to <https://github.com/MinaProtocol/mina/blob/76ecf475e974c0f3c36fedb547e1ff46a9deaa82/src/lib/non_zero_curve_point/non_zero_curve_point.ml#L119>
    fn append(&mut self, value: &CompressedCurvePoint) {
        self.append_field(value.to_big256().into());
        self.append_bit(value.is_odd);
    }
}

impl From<CompressedCurvePoint> for BigInteger256 {
    fn from(p: CompressedCurvePoint) -> BigInteger256 {
        p.to_big256()
    }
}

impl From<CompressedCurvePoint> for Fp {
    fn from(p: CompressedCurvePoint) -> Self {
        let big256: BigInteger256 = p.into();
        big256.into()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PublicKey {
    pub poly: CompressedCurvePoint,
}

impl_bs58_for_binprot!(PublicKey, version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED);

impl RandomOraclePartialInput<PublicKey> for ROInput {
    fn append(&mut self, value: &PublicKey) {
        self.append(&value.poly)
    }
}

// TODO: Replace PublicKey2 usage with PublicKey as they are pretty much the same
// in terms of bin-prot serde
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Deref, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PublicKey2(pub CompressedCurvePoint);

impl_bs58_for_binprot!(PublicKey2, version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED);

impl RandomOraclePartialInput<PublicKey2> for ROInput {
    fn append(&mut self, value: &PublicKey2) {
        self.append(&value.0)
    }
}

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

    fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
        let field_point_bytes: &[u8; 32] = self.0 .0 .0.as_ref();
        output.extend(field_point_bytes);
        let inner_curve_scalar_bytes: &[u8; 32] = self.0 .1 .0.as_ref();
        output.extend(inner_curve_scalar_bytes);
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

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
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
    use num::BigUint;

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
        assert_eq!(s, k.to_base58_string());

        let k = PublicKey2::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58_string());
    }

    // Test cases are generated from OCaml code
    // Run mina code with mainnet profile after adding below code
    // into `load` function in `src/genesis_ledger_helper/genesis_ledger_helper.ml`
    //
    // (* begin CS debugging *)
    //   let padded_accounts =
    //     padded_accounts_opt |> Option.value_exn |> Lazy.force
    //   in
    //   Print.printf "padded_accounts_from_runtime_config_opt: %d\n"
    //     (padded_accounts |> List.length) ;
    //   let _, acc = padded_accounts |> List.hd_exn in
    //   let acc_json = Account.to_yojson acc in
    //   let pk_compressed = acc |> Account.public_key in
    //   let ({ x; is_odd }
    //         : ( Marlin_plonk_bindings_pasta_fp.t
    //           , bool )
    //           Public_key.Compressed.Poly.t) =
    //     pk_compressed
    //   in
    //   let pk_compressed_roinput =
    //     pk_compressed |> Public_key.Compressed.to_input
    //   in
    //   let pk_compressed_ro_packed_fields =
    //     pk_compressed_roinput |> Random_oracle.pack_input
    //   in
    //   Print.printf
    //     "padded_accounts_from_runtime_config_opt[0]: %s\n\
    //      pk_compressed:%s,x:%s,odd:%b\n"
    //     (acc_json |> Yojson.Safe.pretty_to_string)
    //     (pk_compressed |> Public_key.Compressed.to_string)
    //     (x |> Snark_params.Tick.Field.to_string)
    //     is_odd ;
    //   Print.printf "roinput of pk_compressed packed fields:\n" ;
    //   for i = 0 to Array.length pk_compressed_ro_packed_fields - 1 do
    //     Print.printf "\"%s\",\n"
    //       ( pk_compressed_ro_packed_fields.(i)
    //       |> Snark_params.Tick.Field.to_string )
    //   done ;
    //   (* end CS debugging *)
    #[test]
    fn public_key_fields() {
        // This is the public key of the first account in padded_accounts_from_runtime_config_opt
        let s = "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg";
        let pk = PublicKey::from_base58(s).unwrap();
        assert_eq!(s, pk.to_base58_string());
        assert_eq!(pk.poly.is_odd, false);
        assert_eq!(
            field_to_str_radix_10(pk.poly.clone().into()),
            "22536877747820698688010660184495467853785925552441222123266613953322243475471"
        );
        let roinput_fields = {
            let mut roinput = ROInput::new();
            roinput.append(&pk);
            roinput.to_fields()
        };
        assert_eq!(roinput_fields.len(), 2);
        assert_eq!(
            field_to_str_radix_10(roinput_fields[0]),
            "22536877747820698688010660184495467853785925552441222123266613953322243475471"
        );
        assert_eq!(field_to_str_radix_10(roinput_fields[1]), "0");
    }

    #[test]
    fn signature_from_base58_roundtrip() {
        let s = "7mXTB1bcHYLJTmTfMtTboo4FSGStvera3z2wd6qjSxhpz1hZFMZZjcyaWAFEmZhgbq6DqVqGsNodnYKsCbMAq7D8yWo5bRSd";
        let k = Signature::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58_string());
    }

    fn field_to_str_radix_10(f: Fp) -> String {
        let big256: BigInteger256 = f.into();
        let big: BigUint = big256.into();
        big.to_str_radix(10)
    }
}
