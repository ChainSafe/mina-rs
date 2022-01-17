// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub use bin_prot::error::Error;
use bin_prot::Deserializer;
use serde::{Deserialize, Serialize};

pub trait BinProtEncodable {
    const PREALLOCATE_BUFFER_BYTES: usize;

    fn try_encode_binprot(&self) -> Result<Vec<u8>, Error>
    where
        Self: Serialize,
    {
        let mut output = Vec::with_capacity(Self::PREALLOCATE_BUFFER_BYTES);
        bin_prot::to_writer(&mut output, &self)?;
        Ok(output)
    }

    fn try_decode_binprot<'de>(bytes: impl AsRef<[u8]>) -> Result<Self, Error>
    where
        Self: Deserialize<'de> + Sized,
    {
        let mut de = Deserializer::from_reader(bytes.as_ref());
        Deserialize::deserialize(&mut de)
    }
}
