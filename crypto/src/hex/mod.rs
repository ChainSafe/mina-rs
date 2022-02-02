// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use num::Integer;

pub trait HexEncodable {
    type Error;
    fn to_hex_string(&self) -> String;
    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub fn skip_0x_prefix_when_needed(s: &[u8]) -> &[u8] {
    if s[1] == b'x' && (s[0] == b'0' || s[0] == b'\\') {
        &s[2..]
    } else {
        s
    }
}

// Some general impls for useful types

// Any size array of bytes
impl<const N: usize> HexEncodable for [u8; N] {
    type Error = hex::FromHexError;

    fn to_hex_string(&self) -> String
    where
        Self: AsRef<[u8]>,
    {
        hex::encode(self)
    }

    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error> {
        let s = skip_0x_prefix_when_needed(s.as_ref());
        let bytes = hex::decode(s)?;
        let mut r = [0_u8; N];
        r.copy_from_slice(&bytes);
        Ok(r)
    }
}

// Vector of array of bytes
impl<const N: usize> HexEncodable for Vec<[u8; N]> {
    type Error = hex::FromHexError;

    fn to_hex_string(&self) -> String {
        self.into_iter().map(|i| i.to_hex_string()).collect()
    }

    fn try_from_hex(s: impl AsRef<[u8]>) -> Result<Self, Self::Error> {
        let s = skip_0x_prefix_when_needed(s.as_ref());
        let (q, r) = s.len().div_rem(&N);
        let mut vec = Vec::with_capacity(match r > 0 {
            true => q + 1,
            _ => q,
        });
        for chunk in s.chunks(N) {
            vec.push(<[u8; N]>::try_from_hex(chunk)?);
        }
        Ok(vec)
    }
}
