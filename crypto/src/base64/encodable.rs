// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait Base64Encodable {
    fn to_base64(&self) -> String
    where
        Self: AsRef<[u8]>,
    {
        base64::encode(self)
    }

    fn try_from_base64(i: impl AsRef<[u8]>) -> Result<Self, bin_prot::error::Error>
    where
        Self: From<Vec<u8>>,
    {
        let bytes: Vec<u8> = base64::decode(i).map_err(|e| bin_prot::error::Error::Custom {
            message: format!("{:?}", e),
        })?;
        Ok(bytes.into())
    }
}
