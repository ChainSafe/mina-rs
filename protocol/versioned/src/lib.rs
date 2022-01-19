//!
//! Versioned wrapper types for serialization
//! 
//! In the bin-prot Mina wire protocol, each nested type has an associated version. This is to allow for backward 
//! compatibility if parts of the wire protocol change. This simple wrapper type ensures that this information
//! is included in the serialized output in an indentical way to the mina reference implementation.
//!

use::serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Versioned<T, const V: u16> {
    pub version: u16,
    pub t: T,
}

impl<T, const V: u16> Versioned<T, V> {
    pub fn new(t: T) -> Self {
        Self {
            version: V,
            t,
        }
    }

    pub fn inner(self) -> T {
        self.t
    }

    pub fn version(&self) -> u16 {
        self.version
    }

}

impl<T, const V: u16> From<T> for Versioned<T, V> {
    fn from(t: T) -> Versioned<T, V> {
        Self::new(t)
    }
}
