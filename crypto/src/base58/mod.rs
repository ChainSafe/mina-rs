// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

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
