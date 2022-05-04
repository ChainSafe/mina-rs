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

pub mod macros;

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
            version: V, // version should always be equal to V
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
    #[inline]
    fn from(t: T) -> Self {
        Versioned::new(t)
    }
}

impl<T, const V1: u16, const V2: u16> From<T> for Versioned<Versioned<T, V1>, V2> {
    #[inline]
    fn from(t: T) -> Self {
        let t: Versioned<T, V1> = t.into();
        t.into()
    }
}

impl<T, const V1: u16, const V2: u16, const V3: u16> From<T>
    for Versioned<Versioned<Versioned<T, V1>, V2>, V3>
{
    #[inline]
    fn from(t: T) -> Self {
        let t: Versioned<Versioned<T, V1>, V2> = t.into();
        t.into()
    }
}

impl<T, const V1: u16, const V2: u16, const V3: u16, const V4: u16> From<T>
    for Versioned<Versioned<Versioned<Versioned<T, V1>, V2>, V3>, V4>
{
    #[inline]
    fn from(t: T) -> Self {
        let t: Versioned<Versioned<Versioned<T, V1>, V2>, V3> = t.into();
        t.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_versioned() {
        type I32V1 = Versioned<i32, 2>;

        let i = I32V1::default();
        assert_eq!(i.version(), 2);
        assert_eq!(i.inner(), i32::default());
    }
}
