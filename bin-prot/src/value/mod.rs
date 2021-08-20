// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! The Value enum, a loosely typed way of representing any valid bin_prot value.
//!
//! Since bin_prot is not a self describing format, deserializing to a loosely typed value required
//! a supplimentary file that describes the layout of the binary (see layout/)

use serde::Deserialize;
use std::collections::HashMap;

mod enum_data;
mod index;
pub mod layout;
mod visitor;

pub use self::index::Index;
pub use enum_data::EnumData;

use visitor::ValueVisitor;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)] // allow for now until full implementation
pub enum Value {
    Unit,
    Nat0(u32),
    Bool(bool),
    String(Vec<u8>),
    Char(char),
    Int(i64),
    Float(f64),
    Option(Option<Box<Value>>),
    Record(HashMap<String, Value>), // records/structs
    Tuple(Vec<Value>),
    Sum {
        name: String,
        index: u8,
        value: Box<Value>,
    }, // sum types/enums
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
