// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Some basic versioned types used throughout

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use versioned::Versioned;

/// 32 bytes representing a hash of some kind (v1)
pub type HashV1 = Versioned<[u8; 32], 1>;

/// 32 bytes representing a hash of some kind (v1) with extra version byte
pub type Hash2V1 = Versioned<HashV1, 1>;

/// u64 representing a token ID (v1)
pub type TokenIdV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;

/// u64 representing a block time (v1)
pub type BlockTimeV1 = Versioned<Versioned<u64, 1>, 1>;

/// u64 representing an account nonce (v1)
pub type AccountNonceV1 = Versioned<Versioned<u64, 1>, 1>;

/// u32 representing a length (v1)
pub type LengthV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 representing a delta (i.e. difference) (v1)
pub type DeltaV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 representing a slot number (v1)
pub type GlobalSlotNumberV1 = Versioned<Versioned<u32, 1>, 1>;

/// u64 representing an amount of currency (v1)
pub type AmountV1 = Versioned<Versioned<u64, 1>, 1>;

// FIXME: 255 255 cannot be deserialized to u32, use i32 for now
// Note: Extended_Uint32 is not defined in bin_prot, but comes from mina
// Block path: t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/t/t/payload/t/t/common/t/t/t/valid_until
/// u32 wrapped in 1 version byte
pub type ExtendedU32 = Versioned<Versioned<i32, 1>, 1>;

/// u64 wrapped in 1 version byte
pub type ExtendedU64 = Versioned<u64, 1>;

/// u64 wrapped in 2 version bytes
pub type ExtendedU64_2 = Versioned<ExtendedU64, 1>;

/// u64 wrapped in 3 version bytes
pub type ExtendedU64_3 = Versioned<ExtendedU64_2, 1>;

/// Versioned 64 bytes
pub type Hex64V1 = Versioned<i64, 1>;

/// Versioned char
pub type CharV1 = Versioned<u8, 1>;

/// 32 bytes representing a BigInt256
pub type BigInt256 = [u8; 32];

/// Vector of bytes with a version number. Also encodes its own length when encoded using bin-prot
pub type ByteVecV1 = Versioned<Vec<u8>, 1>;

/// A wrapper of versioned type that is base58 encodable with an optional version byte
#[derive(Debug, Clone)]
pub struct Base58EncodableVersionedType<const VERSION_BYTE: u8, T>(pub T);

impl<const VERSION_BYTE: u8, T> Serialize for Base58EncodableVersionedType<VERSION_BYTE, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = Vec::new();
        bin_prot::to_writer(&mut buf, &self.0).map_err(<S::Error as serde::ser::Error>::custom)?;
        let s = bs58::encode(buf)
            .with_check_version(VERSION_BYTE)
            .into_string();
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
        let bytes: Vec<u8> = bs58::decode(s)
            .with_check(Some(VERSION_BYTE))
            .into_vec()
            .map_err(<D::Error as serde::de::Error>::custom)?;
        // skip the version check byte
        let data: T =
            bin_prot::from_reader(&bytes[1..]).map_err(<D::Error as serde::de::Error>::custom)?;
        Ok(Base58EncodableVersionedType::<VERSION_BYTE, T>(data))
    }
}

/// base58 string representation of a hash
pub type HashV1Json<const VERSION_BYTE: u8> = Base58EncodableVersionedType<VERSION_BYTE, HashV1>;
