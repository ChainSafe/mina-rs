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

    fn to_base58_string(&self) -> String
    where
        Self: Sized + Serialize,
    {
        self.to_base58().into_string()
    }

    fn from_base58<'a, I>(i: I) -> Result<Self, bin_prot::error::Error>
    where
        I: AsRef<[u8]>,
        Self: Sized + Deserialize<'a>,
    {
        let bytes: Vec<u8> = decode(i)
            .with_check(Some(Self::VERSION_BYTE))
            .into_vec()
            .map_err(|e| bin_prot::error::Error::Custom {
                message: format!("{:?}", e),
            })?;

        // skip the first byte as this still contains the version byte
        from_reader(&bytes[1..])
    }
}

pub trait Base58EncodableHash {
    /// This is the only part a custom implementation need provide.
    /// Should be a constant from the base58_version_bytes.rs file corresponding
    /// to the type.
    const VERSION_BYTE: u8;
    const MINA_VERSION_BYTE: u8 = 1;

    fn to_base58(&self) -> EncodeBuilder<'static, Vec<u8>>
    where
        Self: AsRef<[u8; 32]>,
    {
        let mut buf = Vec::with_capacity(33);
        buf.push(Self::MINA_VERSION_BYTE);
        for &b in self.as_ref() {
            buf.push(b);
        }
        encode(buf).with_check_version(Self::VERSION_BYTE)
    }

    fn to_base58_string(&self) -> String
    where
        Self: AsRef<[u8; 32]>,
    {
        self.to_base58().into_string()
    }

    fn from_base58<I>(i: I) -> Result<Self, bin_prot::error::Error>
    where
        I: AsRef<[u8]>,
        Self: From<[u8; 32]>,
    {
        let bytes: Vec<u8> = decode(i)
            .with_check(Some(Self::VERSION_BYTE))
            .into_vec()
            .map_err(|e| bin_prot::error::Error::Custom {
                message: format!("{:?}", e),
            })?;

        // skip the bs58 version byte and mina bin_prot version byte
        let mut b32 = [0; 32];
        b32.copy_from_slice(&bytes[2..]);
        Ok(b32.into())
    }
}

#[macro_export]
macro_rules! impl_hash_bs58 {
    ( $ty:ty, $expr:expr ) => {
        impl Base58EncodableHash for $ty {
            const VERSION_BYTE: u8 = $expr;
        }

        impl From<[u8; 32]> for $ty {
            fn from(h: [u8; 32]) -> Self {
                Self(h.into())
            }
        }

        impl AsRef<[u8; 32]> for $ty {
            fn as_ref(&self) -> &[u8; 32] {
                self.0.as_ref()
            }
        }
    };
}
