// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::Deserializer;
use serde::{Deserialize, Serialize};

/// Trait to allow a type to give a size it will serialize to/from
pub trait BinProtEncodable {
    /// Size of buffer to allocate to serialialized form. This required prior knowledge.
    const PREALLOCATE_BUFFER_BYTES: usize;

    /// Tries to encode self into a preallocated buffer of bytes in bin-prot form
    fn try_encode_binprot(&self) -> Result<Vec<u8>, Error>
    where
        Self: Serialize,
    {
        let mut output = Vec::with_capacity(Self::PREALLOCATE_BUFFER_BYTES);
        crate::to_writer(&mut output, &self)?;
        Ok(output)
    }

    /// Try to decode some bytes into the implementing type interpreting them as bin-prot encoding
    fn try_decode_binprot<'de>(bytes: impl AsRef<[u8]>) -> Result<Self, Error>
    where
        Self: Deserialize<'de> + Sized,
    {
        let mut de = Deserializer::from_reader(bytes.as_ref());
        Deserialize::deserialize(&mut de)
    }
}
