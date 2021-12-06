// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait HexEncodable {
    type Error;
    fn to_hex_string(&self) -> String;
    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
