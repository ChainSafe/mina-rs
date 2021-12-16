// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Mina SNARK keys are encoded in a dedicated, self documenting, file format
//!
//! Snark key files have a header and body section
//! The header begins with a string describing the type of snark key (currently always "MINA_SNARK_KEYS")
//! The second line is json formatted and describes the key parameters
//! The remainder is the bin_prot encoded key data of the given type (e.g. VerificatioKey, ProvingKey)
//!
//! For the full specification see <https://github.com/MinaProtocol/mina/blob/f88edb440e321114e26f7691e599adab30ce16cd/docs/specs/types_and_structures/serialized_key.md>

mod error;
mod header;
mod reader;

pub use error::{Error, Result};
pub use header::*;
pub use reader::read_snark_key_file;
