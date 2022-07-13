// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::io::Read;

use serde::{Deserialize, Serialize};

/// This trait annotates a given type its corresponding bin-prot serialization type,
pub trait BinProtSerializationType<'de>: Sized {
    /// The corresponding serialization type for bin-prot format
    /// Self type can be used here to indicate no special convertion is needed
    /// TODO: Add default value when the feature lands on stable rust
    type T: From<Self> + Into<Self> + Serialize + Deserialize<'de>;

    /// Construct from binprot bytes reader
    fn try_from_binprot(rdr: impl Read) -> Result<Self, bin_prot::error::Error> {
        let binprot: Self::T = bin_prot::from_reader_strict(rdr)?;
        Ok(binprot.into())
    }

    /// Convert into binprot byte vec
    fn try_into_binprot(self) -> Result<Vec<u8>, bin_prot::error::Error> {
        let binprot: Self::T = self.into();
        let mut bytes = Vec::new();
        bin_prot::to_writer(&mut bytes, &binprot)?;
        Ok(bytes)
    }
}

/// This trait annotates a given type its corresponding json serialization type,
/// and provide utility functions to easily convert between them
pub trait JsonSerializationType<'de>: Sized {
    /// The corresponding serialization type for json format
    /// Self type can be used here to indicate no special convertion is needed
    /// TODO: Add default value when the feature lands on stable rust
    type T: From<Self> + Into<Self> + Serialize + Deserialize<'de>;

    /// Construct from json string
    fn try_from_json(s: &'de str) -> Result<Self, serde_json::error::Error> {
        let json: Self::T = serde_json::from_str(s)?;
        Ok(json.into())
    }

    /// Convert into json string
    fn try_into_json(self) -> Result<String, serde_json::error::Error> {
        let json: Self::T = self.into();
        serde_json::to_string(&json)
    }
}
