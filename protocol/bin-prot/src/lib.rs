// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(rustdoc::all)]

//!
//! This crate implements <https://github.com/janestreet/bin_prot> in rust
//!

mod array;
mod consts;
mod de;
pub mod error;
pub mod integers;
mod loose_deserializer;
mod read_ext;
mod ser;
pub mod value;
mod write_ext;

pub use array::OcamlArray;
pub use de::{from_reader, Deserializer};
pub use read_ext::ReadBinProtExt;
pub use ser::{to_writer, Serializer};
pub use value::layout::{BinProtRule, Layout};
pub use value::Value;
pub use write_ext::WriteBinProtExt;
