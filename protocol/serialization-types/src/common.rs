// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Some basic versioned types used throughout

use crate::version_bytes;
use bs58::encode::EncodeBuilder;
use derive_more::{From, Into};
use mina_serialization_types_macros::AutoFrom;
use num::Integer;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use versioned::*;

/// 32 bytes representing a hash of some kind (v1)
pub type HashV1 = Versioned<[u8; 32], 1>;

/// 32 bytes representing a hash of some kind (v1) with extra version byte
pub type Hash2V1 = Versioned<HashV1, 1>;

/// u64 representing a token ID (v1)
pub type TokenIdV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;
impl_from_for_newtype!(U64Json, TokenIdV1);

/// u64 representing a block time (v1)
pub type BlockTimeV1 = Versioned<Versioned<u64, 1>, 1>;

/// u64 representing an account nonce (v1) // This should also be an extendedu32
pub type AccountNonceV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 wrapper (json)
/// Note that integers are represented as string in mina json
#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct U32Json(pub u32);

impl Serialize for U32Json {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for U32Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(Self(
            s.parse().map_err(<D::Error as serde::de::Error>::custom)?,
        ))
    }
}

/// u64 wrapper (json)
/// Note that integers are represented as string in mina json
#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct U64Json(pub u64);

impl Serialize for U64Json {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for U64Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(Self(
            s.parse().map_err(<D::Error as serde::de::Error>::custom)?,
        ))
    }
}

/// i64 wrapper (json)
#[derive(
    Clone, Debug, Eq, PartialEq, Serialize, Deserialize, derive_more::From, derive_more::Into,
)]
pub struct I64(pub i64);

/// u64 wrapper (json)
/// Note that integers are represented as string in mina json
#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct DecimalJson(pub u64);

const MINA_PRECISION: u64 = 1000000000;
impl Serialize for DecimalJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (q, r) = self.0.div_rem(&MINA_PRECISION);
        let s = format!("{q}.{:0>9}", r);
        serializer.serialize_str(s.trim_end_matches('0').trim_end_matches('.'))
    }
}

impl<'de> Deserialize<'de> for DecimalJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut iter = s.split('.');
        let q: u64 = iter
            .next()
            .ok_or_else(|| <D::Error as serde::de::Error>::custom(format!("Invalid string: {s}")))?
            .parse()
            .map_err(<D::Error as serde::de::Error>::custom)?;
        let r_str = iter.next().unwrap_or("0");
        let r: u64 = format!("{:0<9}", r_str)
            .parse()
            .map_err(<D::Error as serde::de::Error>::custom)?;
        if iter.next().is_none() {
            // ensure there isn't more to parse as that is undefined
            Ok(Self(r + MINA_PRECISION * q))
        } else {
            Err(<D::Error as serde::de::Error>::custom(format!(
                "Invalid string: {s}"
            )))
        }
    }
}

/// u32 representing a length (v1)
pub type LengthV1 = Versioned<Versioned<u32, 1>, 1>;
impl_from_for_newtype!(U32Json, LengthV1);

/// u32 representing a delta (i.e. difference) (v1)
pub type DeltaV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 representing a slot number (v1)
pub type GlobalSlotNumberV1 = Versioned<Versioned<u32, 1>, 1>;

/// u64 representing an amount of currency (v1)
pub type AmountV1 = Versioned<Versioned<u64, 1>, 1>;
impl_from_for_newtype!(U64Json, AmountV1);
impl_from_for_newtype!(DecimalJson, AmountV1);

// FIXME: 255 255 cannot be deserialized to u32, use i32 for now
// Note: Extended_Uint32 is not defined in bin_prot, but comes from mina
// Block path: t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/t/t/payload/t/t/common/t/t/t/valid_until
/// u32 wrapped in 1 version byte
pub type ExtendedU32 = Versioned<Versioned<i32, 1>, 1>;
impl From<U32Json> for ExtendedU32 {
    fn from(t: U32Json) -> Self {
        (t.0 as i32).into()
    }
}

impl From<ExtendedU32> for U32Json {
    fn from(t: ExtendedU32) -> Self {
        Self(t.t.t as u32)
    }
}

/// u64 wrapped in 1 version byte
pub type ExtendedU64 = Versioned<u64, 1>;

/// u64 wrapped in 2 version bytes
pub type ExtendedU64_2 = Versioned<ExtendedU64, 1>;

/// u64 wrapped in 3 version bytes
pub type ExtendedU64_3 = Versioned<ExtendedU64_2, 1>;

/// Versioned 64 bytes
pub type Hex64V1 = Versioned<i64, 1>;
impl_from_for_newtype!(I64, Hex64V1);

/// char (v1)
pub type CharV1 = Versioned<u8, 1>;
impl_from_for_newtype!(CharJson, CharV1);

/// char (json)
#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct CharJson(pub u8);

impl Serialize for CharJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = unsafe { String::from_utf8_unchecked(vec![self.0]) };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for CharJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.as_bytes()[0].into())
    }
}

/// 32 bytes representing a BigInt256
pub type BigInt256 = [u8; 32];

/// Wrapper of Vec<u8>
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, derive_more::From)]
pub struct ByteVec(pub Vec<u8>);

impl_from_versioned!(ByteVec);

/// Wrapper of Vec<u8> (json)
#[derive(Debug, Clone, Eq, PartialEq, From, Into, AutoFrom)]
#[auto_from(ByteVec)]
pub struct ByteVecJson(pub Vec<u8>);

impl Serialize for ByteVecJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = unsafe { String::from_utf8_unchecked(self.0.clone()) };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ByteVecJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let v: Vec<u8> = s.as_bytes().to_vec();
        Ok(Self(v))
    }
}

/// Vector of bytes with a version number. Also encodes its own length when encoded using bin-prot
pub type ByteVecV1 = Versioned<ByteVec, 1>;

/// A wrapper of versioned type that is base58 encodable
#[derive(Debug, Clone, Eq, PartialEq, derive_more::From)]
pub struct Base58EncodableType<const VERSION_BYTE: u8, T>(pub T);

impl<const VERSION_BYTE: u8, T> Base58EncodableType<VERSION_BYTE, T>
where
    T: From<Vec<u8>>,
{
    /// Decode input base58 encoded bytes into [Base58EncodableType]
    pub fn from_base58(input: impl AsRef<[u8]>) -> Result<Self, crate::errors::Error> {
        let bytes: Vec<u8> = bs58::decode(input)
            .with_check(Some(VERSION_BYTE))
            .into_vec()
            .map_err(crate::errors::Error::Base58DecodeError)?;
        // skip the version check byte
        let mut v: Vec<u8> = Vec::with_capacity(bytes.len() - 1);
        v.extend_from_slice(&bytes[1..]);
        Ok(Self(v.into()))
    }
}

impl<const VERSION_BYTE: u8, T> Base58EncodableType<VERSION_BYTE, T>
where
    T: Serialize + AsRef<[u8]>,
{
    /// Encode inner data with version check byte into [String]
    pub fn to_base58_string(&self) -> Result<String, crate::errors::Error> {
        Ok(self.to_base58_builder().into_string())
    }

    /// Encode inner data with version check byte into [EncodeBuilder]
    pub fn to_base58_builder(&self) -> EncodeBuilder<'static, &[u8]> {
        let bytes: &[u8] = self.0.as_ref();
        bs58::encode(bytes).with_check_version(VERSION_BYTE)
    }
}

impl<const VERSION_BYTE: u8, T> From<Base58EncodableType<VERSION_BYTE, T>> for (T,) {
    fn from(i: Base58EncodableType<VERSION_BYTE, T>) -> Self {
        (i.0,)
    }
}

impl<const VERSION_BYTE: u8, T> Serialize for Base58EncodableType<VERSION_BYTE, T>
where
    T: Serialize + AsRef<[u8]>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self
            .to_base58_string()
            .map_err(<S::Error as serde::ser::Error>::custom)?;
        serializer.serialize_str(&s)
    }
}

impl<'de, const VERSION_BYTE: u8, T> Deserialize<'de> for Base58EncodableType<VERSION_BYTE, T>
where
    T: Deserialize<'de> + From<Vec<u8>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s =
            String::deserialize(deserializer).map_err(<D::Error as serde::de::Error>::custom)?;
        Self::from_base58(s).map_err(<D::Error as serde::de::Error>::custom)
    }
}

/// A wrapper of versioned type that is base58 encodable with an version byte
#[derive(Debug, Clone, Eq, PartialEq, derive_more::From)]
pub struct Base58EncodableVersionedType<const VERSION_BYTE: u8, T>(pub T);

impl<'de, const VERSION_BYTE: u8, T> Base58EncodableVersionedType<VERSION_BYTE, T>
where
    T: Deserialize<'de>,
{
    /// Decode input base58 encoded bytes into [Base58EncodableVersionedType]
    pub fn from_base58(input: impl AsRef<[u8]>) -> Result<Self, crate::errors::Error> {
        let bytes: Vec<u8> = bs58::decode(input)
            .with_check(Some(VERSION_BYTE))
            .into_vec()
            .map_err(crate::errors::Error::Base58DecodeError)?;
        // skip the version check byte
        let data: T = bin_prot::from_reader_strict(&bytes[1..])
            .map_err(crate::errors::Error::BinProtError)?;
        Ok(Self(data))
    }
}

impl<const VERSION_BYTE: u8, T> Base58EncodableVersionedType<VERSION_BYTE, T>
where
    T: Serialize,
{
    /// Encode inner data with version check byte into [String]
    pub fn to_base58_string(&self) -> Result<String, crate::errors::Error> {
        let builder = self.to_base58_builder()?;
        Ok(builder.into_string())
    }

    /// Encode inner data with version check byte into [EncodeBuilder]
    pub fn to_base58_builder(
        &self,
    ) -> Result<EncodeBuilder<'static, Vec<u8>>, bin_prot::error::Error> {
        let mut buf = Vec::new();
        bin_prot::to_writer(&mut buf, &self.0)?;
        Ok(bs58::encode(buf).with_check_version(VERSION_BYTE))
    }
}

impl<const VERSION_BYTE: u8, T> From<Base58EncodableVersionedType<VERSION_BYTE, T>> for (T,) {
    fn from(i: Base58EncodableVersionedType<VERSION_BYTE, T>) -> Self {
        (i.0,)
    }
}

impl<const VERSION_BYTE: u8, T> Serialize for Base58EncodableVersionedType<VERSION_BYTE, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self
            .to_base58_string()
            .map_err(<S::Error as serde::ser::Error>::custom)?;
        serializer.serialize_str(&s)
    }
}

impl<'de, const VERSION_BYTE: u8, T> Deserialize<'de>
    for Base58EncodableVersionedType<VERSION_BYTE, T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s =
            String::deserialize(deserializer).map_err(<D::Error as serde::de::Error>::custom)?;
        Self::from_base58(s).map_err(<D::Error as serde::de::Error>::custom)
    }
}

/// base58 string representation of a hash
pub type HashV1Json<const VERSION_BYTE: u8> = Base58EncodableVersionedType<VERSION_BYTE, HashV1>;

impl<const VERSION_BYTE: u8> From<HashV1Json<VERSION_BYTE>> for HashV1 {
    fn from(i: HashV1Json<VERSION_BYTE>) -> Self {
        let (h,) = i.into();
        h
    }
}

impl<const VERSION_BYTE: u8> From<HashV1Json<VERSION_BYTE>> for Hash2V1 {
    fn from(i: HashV1Json<VERSION_BYTE>) -> Self {
        let v1: HashV1 = i.into();
        v1.into()
    }
}

impl<const VERSION_BYTE: u8> From<Hash2V1> for HashV1Json<VERSION_BYTE> {
    fn from(i: Hash2V1) -> Self {
        i.t.into()
    }
}

/// base58 string representation of a ledger hash
pub type LedgerHashV1Json = HashV1Json<{ version_bytes::LEDGER_HASH }>;

/// base58 string representation of a chain hash
pub type ChainHashV1Json = LedgerHashV1Json;

/// base58 string representation of a coinbase hash
pub type CoinBaseHashV1Json = HashV1Json<{ version_bytes::COINBASE_HASH }>;

/// base58 string representation of a coinbase hash
pub type EpochSeedHashV1Json = HashV1Json<{ version_bytes::EPOCH_SEED }>;

/// base58 string representation of a state hash
pub type StateHashV1Json = HashV1Json<{ version_bytes::STATE_HASH }>;

/// base58 string representation of a coinbase stack data hash
pub type CoinBaseStackDataV1Json = HashV1Json<{ version_bytes::COINBASE_STACK_DATA }>;

/// base58 string representation of a coinbase stack hash
pub type CoinBaseStackHashV1Json = HashV1Json<{ version_bytes::COINBASE_STACK_HASH }>;

/// base58 string representation of a vrf output hash
pub type VrfOutputHashV1Json = HashV1Json<{ version_bytes::VRF_TRUNCATED_OUTPUT }>;

/// base58 string representation of a aux hash
pub type AuxHashJson = Base58EncodableType<{ version_bytes::STAGED_LEDGER_HASH_AUX_HASH }, Vec<u8>>;

/// base58 string representation of a pending coinbase aux hash
pub type PendingCoinbaseAuxHashJson =
    Base58EncodableType<{ version_bytes::STAGED_LEDGER_HASH_PENDING_COINBASE_AUX }, Vec<u8>>;

impl<const VERSION_BYTE: u8> From<Base58EncodableType<VERSION_BYTE, Vec<u8>>> for ByteVecV1 {
    fn from(i: Base58EncodableType<VERSION_BYTE, Vec<u8>>) -> Self {
        let bv: ByteVec = i.0.into();
        bv.into()
    }
}

impl<const VERSION_BYTE: u8> From<ByteVecV1> for Base58EncodableType<VERSION_BYTE, Vec<u8>> {
    fn from(i: ByteVecV1) -> Self {
        let bv: ByteVec = i.into();
        bv.0.into()
    }
}
