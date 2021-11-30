// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::de::Deserializer;
use serde::{Deserialize, Serialize};

pub trait BinProtDeser {
    const PREALLOCATE_BUFFER_BYTES: usize;

    fn try_serialize(&self) -> Result<Vec<u8>, crate::error::Error>
    where
        Self: Serialize,
    {
        let mut output = Vec::with_capacity(Self::PREALLOCATE_BUFFER_BYTES);
        crate::to_writer(&mut output, &self)?;
        Ok(output)
    }

    fn try_deserialize<'de>(bytes: impl AsRef<[u8]>) -> Result<Self, crate::error::Error>
    where
        Self: Deserialize<'de> + Sized,
    {
        let mut de = Deserializer::from_reader(bytes.as_ref());
        Deserialize::deserialize(&mut de)
    }
}
