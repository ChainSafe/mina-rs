// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Versioned wrapper types for serialization
//!
//! In the bin-prot Mina wire protocol, each nested type has an associated version. This is to allow for backward
//! compatibility if parts of the wire protocol change. This simple wrapper type ensures that this information
//! is included in the serialized output in an indentical way to the mina reference implementation.
//!

#![deny(warnings)]
#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// A generic version wrapper around another type
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Versioned<T, const V: u16> {
    /// Version byte to be encoded first when the whole wrapper is serialized
    pub version: u16,
    /// The wrapped type
    pub t: T,
}

impl<T, const V: u16> Default for Versioned<T, V>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            version: 1_u16, // 1 is the default version number
            t: Default::default(),
        }
    }
}

impl<T, const V: u16> Versioned<T, V> {
    /// create a new version type of the given const version
    pub fn new(t: T) -> Self {
        Self { version: V, t }
    }

    /// Return the inner type
    pub fn inner(self) -> T {
        self.t
    }

    /// Return the version number
    pub fn version(&self) -> u16 {
        self.version
    }
}

impl<T, const V: u16> From<T> for Versioned<T, V> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}
