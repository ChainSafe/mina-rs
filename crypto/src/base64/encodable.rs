// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub trait Base64Encodable {
    fn to_base64(&self) -> String
    where
        Self: AsRef<[u8]>,
    {
        base64::encode(self)
    }

    fn try_from_base64(i: impl AsRef<[u8]>) -> Result<Self, Error>
    where
        Self: From<Vec<u8>>,
    {
        let bytes: Vec<u8> = base64::decode(i).map_err(Error::Base64DecodeError)?;
        Ok(bytes.into())
    }
}
