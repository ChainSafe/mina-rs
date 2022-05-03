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
macro_rules! impl_bs58_json {
    ($ty:ty, $ty_json:ty) => {
        impl $ty {
            pub fn from_base58(
                input: impl AsRef<[u8]>,
            ) -> Result<Self, mina_serialization_types::errors::Error> {
                let t = <$ty_json>::from_base58(input)?;
                Ok(t.into())
            }

            pub fn into_base58_string(
                self,
            ) -> Result<String, mina_serialization_types::errors::Error> {
                let t: $ty_json = self.into();
                t.to_base58_string()
            }

            pub fn to_base58_string(
                &self,
            ) -> Result<String, mina_serialization_types::errors::Error> {
                self.clone().into_base58_string()
            }
        }
    };
}
