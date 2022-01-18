// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

// mod array;
mod consts;
mod de;
pub mod error;
pub mod integers;
#[cfg(feature = "loose_deserialization")]
mod loose_deserializer;
mod read_ext;
mod ser;
pub mod value;
mod write_ext;

// pub use array::OcamlArray;
pub use de::{from_reader, Deserializer};
pub use read_ext::ReadBinProtExt;
pub use ser::{to_writer, Serializer};
#[cfg(feature = "loose_deserialization")]
pub use value::layout::{BinProtRule, Layout};
pub use value::Value;
pub use write_ext::WriteBinProtExt;
