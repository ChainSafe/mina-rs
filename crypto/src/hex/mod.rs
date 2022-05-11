// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait HexEncodable {
    type Error;
    fn to_hex_string(&self) -> String;
    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub fn skip_0x_prefix_when_needed(s: &[u8]) -> &[u8] {
    if s.len() > 1 && s[1] == b'x' && (s[0] == b'0' || s[0] == b'\\') {
        &s[2..]
    } else {
        s
    }
}
