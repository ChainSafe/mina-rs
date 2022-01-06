// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use bs58::encode::EncodeBuilder;

pub trait Base58Encodable {
    /// This is the only part a custom implementation need provide.
    /// Should be a constant from the base58_version_bytes.rs file corresponding
    /// to the type.
    const VERSION_BYTE: u8;
    const MINA_VERSION_BYTE_COUNT: usize = 1;
    const MINA_VERSION_BYTE: u8 = 1;
    fn to_base58(&self) -> EncodeBuilder<'static, Vec<u8>> {
        let mut buf = Vec::with_capacity(32 + Self::MINA_VERSION_BYTE_COUNT);
        for _i in 0..Self::MINA_VERSION_BYTE_COUNT {
            buf.push(Self::MINA_VERSION_BYTE);
        }
        self.write_encodable_bytes(&mut buf);
        bs58::encode(buf).with_check_version(Self::VERSION_BYTE)
    }

    fn to_base58_string(&self) -> String {
        self.to_base58().into_string()
    }

    fn from_base58(i: impl AsRef<[u8]>) -> Result<Self, Error>
    where
        Self: TryFrom<Vec<u8>>,
        <Self as TryFrom<std::vec::Vec<u8>>>::Error: std::fmt::Debug,
    {
        let bytes: Vec<u8> = bs58::decode(i)
            .with_check(Some(Self::VERSION_BYTE))
            .into_vec()
            .map_err(|e| Error::OtherError(format!("{:?}", e)))?;

        // skip the bs58 version byte and mina bin_prot version bytes
        let stored_bytes: Vec<u8> = bytes
            .into_iter()
            .skip(1 + Self::MINA_VERSION_BYTE_COUNT)
            .collect();
        stored_bytes
            .try_into()
            .map_err(|e| Error::OtherError(format!("{:?}", e)))
    }

    fn write_encodable_bytes(&self, output: &mut Vec<u8>);
}

#[macro_export]
macro_rules! impl_bs58_for_binprot {
    ($ty:ty, $expr:expr) => {
        impl Base58Encodable for $ty {
            const VERSION_BYTE: u8 = $expr;
            const MINA_VERSION_BYTE_COUNT: usize = 0;

            fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
                bin_prot::to_writer(output, self)
                    .expect("Failed to serialize struct into binprot format");
            }
        }

        impl TryFrom<Vec<u8>> for $ty {
            type Error = bin_prot::error::Error;
            fn try_from(h: Vec<u8>) -> Result<Self, Self::Error> {
                bin_prot::from_reader(h.as_slice())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bs58_full {
    ($ty:ty, $expr:expr, $expr2:expr) => {
        impl Base58Encodable for $ty {
            const VERSION_BYTE: u8 = $expr;
            const MINA_VERSION_BYTE_COUNT: usize = $expr2;

            fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
                output.extend(self.as_ref());
            }
        }

        impl From<Vec<u8>> for $ty {
            fn from(h: Vec<u8>) -> Self {
                let mut b32 = [0; 32];
                b32.copy_from_slice(h.as_slice());
                Self(b32.into())
            }
        }

        impl AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bs58 {
    ($ty:ty, $expr:expr) => {
        crate::impl_bs58_full!($ty, $expr, 1);
    };
}
