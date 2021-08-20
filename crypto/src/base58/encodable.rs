// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::{from_reader, to_writer};
use bs58::encode::EncodeBuilder;
use serde::{Deserialize, Serialize};

pub use bs58::decode::Error;
pub use bs58::{decode, encode};

pub trait Base58Encodable {
    /// This is the only part a custom implementation need provide.
    /// Should be a constant from the base58_version_bytes.rs file corresponding
    /// to the type.
    const VERSION_BYTE: u8;

    fn to_base58(&self) -> EncodeBuilder<'static, Vec<u8>>
    where
        Self: Sized + Serialize,
    {
        let mut buf = Vec::<u8>::new();
        to_writer(&mut buf, self).unwrap();
        encode(buf).with_check_version(Self::VERSION_BYTE)
    }

    fn from_base58<'a, I>(i: I) -> Result<Self, Error>
    where
        I: AsRef<[u8]>,
        Self: Sized + Deserialize<'a>,
    {
        let bytes: Vec<u8> = decode(i).with_check(Some(Self::VERSION_BYTE)).into_vec()?;

        // skip the first byte as this still contains the version byte
        Ok(from_reader(&bytes[1..]).unwrap())
    }
}
