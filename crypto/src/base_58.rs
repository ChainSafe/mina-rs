// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Module contains helpers for adding implementations to/from Base58 encoding
//! for Mina types. Different types have a different byte prefix to prevent confusion
//! when dealing with the opaque encoded types. This module also contains definitions of
//! these prefix bytes as defined in the Mina reference implementation.
//!

use bs58::encode::EncodeBuilder;
use serde::{Deserialize, Serialize};
use serde_bin_prot::{from_reader, to_writer};
use thiserror::Error;

pub use bs58::{decode, encode};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error decoding Base58: {0}")]
    Base58DecodingError(#[from] bs58::decode::Error),
    #[error("Error interpreting bin_prot from decoded bytes: {0}")]
    BinProtDecodingError(#[from] serde_bin_prot::error::Error),
}

/// Implementing this trait on a type allows it to be converted to/from Base58 encoding with versioning and error checking.
/// When implementing this on a type typically only the `version_byte` method needs to be implemented
/// and this should return a byte prefix defined in the base58_version_bytes module.
pub trait MinaBase58 {
    /// This is the only method a custom implementation need provide.
    /// Should be a constant from the base58_version_bytes.rs file corresponding
    /// to the type.
    fn version_byte() -> u8;

    fn to_base58(&self) -> EncodeBuilder<'static, Vec<u8>>
    where
        Self: Sized + Serialize,
    {
        let mut buf = Vec::new();
        // Safe to unwrap here as writing to a in-memory buffer cannot fail.
        to_writer(&mut buf, self).unwrap();
        encode(buf).with_check_version(Self::version_byte())
    }

    fn from_base58<'a, I>(i: I) -> Result<Self, Error>
    where
        I: AsRef<[u8]>,
        Self: Sized + Deserialize<'a>,
    {
        let bytes: Vec<u8> = decode(i)
            .with_check(Some(Self::version_byte()))
            .into_vec()?;

        // skip the first byte as this still contains the version byte
        Ok(from_reader(&bytes[1..])?)
    }
}
