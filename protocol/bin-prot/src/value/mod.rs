// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! The Value enum, a loosely typed way of representing any valid bin_prot value.
//!
//! Since bin_prot is not a self describing format, deserializing to a loosely typed value required
//! a supplimentary file that describes the layout of the binary (see layout/)

use serde::Deserialize;

mod enum_data;
mod index;
pub mod layout;
pub mod ser;
mod visitor;

pub use self::index::Index;
pub use enum_data::EnumData;

use visitor::ValueVisitor;

#[derive(Clone, Debug, PartialEq)]
/// Recursive enum that can define any BinProt serializable type
pub enum Value {
    /// Unit type
    Unit,
    /// Variable length natural integer
    Nat0(u32),
    /// Boolean
    Bool(bool),
    /// String
    String(Vec<u8>),
    /// Char
    Char(u8),
    /// Variable length integer
    Int(i64),
    /// Float
    Float(f64),
    /// Option
    Option(Option<Box<Value>>),
    /// records/structs
    Record(Vec<(String, Value)>), 
    /// Tuples (fixed length list)
    Tuple(Vec<Value>),
    /// Sum/enum types
    Sum {
        /// summand/variant name
        name: String,
        /// summand index
        index: u8,
        /// value wrapped by variant
        value: Box<Value>,
    },
    /// List of types (variable length)
    List(Vec<Value>),
}

impl Default for Value {
    fn default() -> Value {
        Value::Unit
    }
}

// Ensure the value visitor is always used when deserializing to a Value (see visitor.rs)
//
// This will always request `deserialize_any` be called since the Value implementation
// does not describe its own structure. Attempting to deserialize into Value from a
// non-self describing format will result in an error
impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

impl Value {
    /// Inner reveals an Option variant as a Rust Option type
    /// Calling inner on a non-option variant results in a panic
    pub fn inner(&self) -> Option<Self> {
        if let Value::Option(inner) = self {
            inner.clone().map(|e| *e)
        } else {
            panic!("Called inner on a non-option variant {:?}", self)
        }
    }
}
