// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Some basic versioned types used throughout

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use versioned::Versioned;

use crate::version_bytes;

/// 32 bytes representing a hash of some kind (v1)
pub type HashV1 = Versioned<[u8; 32], 1>;

/// 32 bytes representing a hash of some kind (v1) with extra version byte
pub type Hash2V1 = Versioned<HashV1, 1>;

/// u64 representing a token ID (v1)
pub type TokenIdV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;

/// u64 representing a block time (v1)
pub type BlockTimeV1 = Versioned<Versioned<u64, 1>, 1>;

/// u64 representing an account nonce (v1) // This should also be an extendedu32
pub type AccountNonceV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 representing a length (v1)
pub type LengthV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 representing a delta (i.e. difference) (v1)
pub type DeltaV1 = Versioned<Versioned<u32, 1>, 1>;

/// u32 representing a slot number (v1)
pub type GlobalSlotNumberV1 = Versioned<Versioned<u32, 1>, 1>;

/// u64 representing an amount of currency (v1)
pub type AmountV1 = Versioned<Versioned<u64, 1>, 1>;

/// Versioned 64 bytes
pub type Hex64V1 = Versioned<i64, 1>;

/// Versioned char
pub type CharV1 = Versioned<u8, 1>;

/// 32 bytes representing a BigInt256
pub type BigInt256 = [u8; 32];

/// Vector of bytes with a version number. Also encodes its own length when encoded using bin-prot
pub type ByteVecV1 = Versioned<Vec<u8>, 1>;

/// A wrapper of versioned type that is base58 encodable with an optional version byte
#[derive(Debug, Clone, PartialEq, derive_more::From)]
pub struct Base58EncodableVersionedType<const VERSION_BYTE: u8, T>(pub T);

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

/// base58 string representation of a ledger hash
pub type LedgerHashV1Json = HashV1Json<{ version_bytes::LEDGER_HASH }>;

/// base58 string representation of a chain hash
pub type ChainHashV1Json = LedgerHashV1Json;

/// base58 string representation of a coinbase hash
pub type CoinBaseHashV1Json = HashV1Json<{ version_bytes::COINBASE_HASH }>;

/// base58 string representation of a coinbase hash
pub type EpochSeedHashV1Json = HashV1Json<{ version_bytes::EPOCH_SEED }>;
