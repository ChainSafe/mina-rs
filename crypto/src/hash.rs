// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! A trait that any type can implement to be able to produce a valid Mina hash
//!

use crate::hash_prefixes::HashPrefix;
use digest::Digest;
use generic_array::GenericArray;
use serde::Serialize;
use serde_bin_prot::to_writer;
use sha2::Sha256;
use std::io::Write;

/// Any internal type that needs to be hashed must implement this trait
/// Since each type should have its own HashType the implementation is generic over the output type
///
/// Only the `prefix()` function should be implemented in most cases
///
/// Implementation can also specify a different hash algorithm (default: Sha256)
/// Is generic over output size as long as OutputType supports conversion from a GenericArray of that size
pub trait MinaHash<OutputType, Hasher = Sha256>: Sized + Serialize
where
    OutputType: Default + From<GenericArray<u8, Hasher::OutputSize>>,
    Hasher: Digest,
{
    /// Must return the conventional hash prefix defined in hash_prefixes.rs
    fn prefix() -> HashPrefix;

    fn hash(&self) -> OutputType {
        let mut buf = Vec::<u8>::new();
        // write the prefix bytes
        buf.write(Self::prefix());
        // write the data bytes
        to_writer(&mut buf, self);
        // compute the hash
        let hash_bytes = Hasher::digest(&buf);
        OutputType::from(hash_bytes)
    }
}
