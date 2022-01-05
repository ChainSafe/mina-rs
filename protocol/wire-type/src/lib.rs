// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(missing_docs)]

//! # Wire Type
//!
//!  `wire-type` exports a trait and a corresponding derive macro to annotate types in Mina-rs
//!  that are to be sent over the wire (must be supported over the network and between node implementations and versions)
//!  
//!  Types that implement WireType have a version number and can be converted into the associated WireType that
//!  includes this version number as a field
//!  
//!  Using WireType is easy! Mostly it will be used via the derive macro
//!  
//!  ```ignore
//!  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
//!  struct X {
//!      a: u32,
//!  }
//!  // X can now be converted into a type X::WireType generated by the macro
//!  // this wraps X in a t field and has a version field. When serialized this
//!  // will match the serialization used in Mina OCaml
//!  ```
//!  
//!  It is also possible to have types that implement WireType automatically convert to and from their
//!  versioned form when they are serialized or deserialized using serde
//!  ```ignore
//!  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
//!  #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
//!  #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
//!  struct X {
//!      a: u32,
//!  }
//!  ```
//!  
//!  By default the version number is 1, This can be changed by adding the attribute
//!  ```ignore
//!  #[wire_type( version = 1)]
//!  ```
//!  to any type that uses the derive.
//!  

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub use wire_type_derive::*;

/// Wire type version type
pub type Version = u16;

/// Types implementing WireType provide an ascociated wire type and implementations to convert
/// to and from this type.
///
/// This is typically produced by the derive macro but can also be implemented manually
///
pub trait WireType<'a>:
    Debug + PartialEq + Serialize + Deserialize<'a> + From<Self::WireType>
{
    /// Associated wire type for this type
    type WireType: Serialize + Deserialize<'a>;
    /// Version of the serialization of this type
    const VERSION: Version;

    /// Convert this type to its wire format
    fn to_wire_type(self) -> Self::WireType;
    /// Convert this type from its wire format
    fn from_wire_type(t: Self::WireType) -> Self;
}

#[cfg(test)]
mod tests {
    use crate as wire_type;
    use serde::{Deserialize, Serialize};
    use wire_type::{Version, WireType};

    #[test]
    fn smoke() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct X {
            a: u32,
        }

        #[derive(Serialize, Deserialize)]
        struct WireX {
            version: Version,
            t: X,
        }

        impl From<WireX> for X {
            fn from(t: WireX) -> Self {
                t.t
            }
        }

        impl<'a> WireType<'_> for X {
            type WireType = WireX;
            const VERSION: Version = 0x01;
            fn to_wire_type(self) -> Self::WireType {
                Self::WireType {
                    version: Self::VERSION,
                    t: self,
                }
            }
            fn from_wire_type(t: Self::WireType) -> Self {
                Self::from(t)
            }
        }
    }

    #[test]
    fn derive() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        struct X {
            a: u32,
        }

        type WireX<'a> = <X as WireType<'a>>::WireType;

        let x = X { a: 123 };

        assert_eq!(X::VERSION, 1);
        assert_eq!(
            x.clone().to_wire_type(),
            WireX {
                version: 1,
                t: x.clone()
            }
        );
        assert_eq!(
            X::from_wire_type(WireX {
                version: 1,
                t: x.clone()
            }),
            x
        )
    }

    #[test]
    fn compatible_with_serde_from_and_into_named_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[wire_type(version = 2)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X {
            a: u32,
            pub b: u32,
        }

        let x = X { a: 123, b: 321 };
        let serialized = serde_json::to_string(&x).unwrap();
        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, deserialized);
    }

    #[test]
    fn compatible_with_serde_from_and_into_unnamed_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X(u32, u32);

        let x = X(123, 321);
        let serialized = serde_json::to_string(&x).unwrap();
        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, deserialized);
    }

    #[test]
    fn compatible_with_serde_from_and_into_unit_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X;

        let x = X;
        let serialized = serde_json::to_string(&x).unwrap();
        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, deserialized);
    }

    #[test]
    fn compatible_with_serde_from_and_into_enum() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        enum X {
            X1(u32),
        }
        let x = X::X1(123);
        let serialized = serde_json::to_string(&x).unwrap();
        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, deserialized);
    }

    #[test]
    fn recursive_depth_named_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[wire_type(recurse = 2)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X {
            pub a: u32,
            b: u32,
        }

        let x = X { a: 123, b: 321 };
        let serialized = serde_json::to_string(&x).unwrap();
        println!("{}", serialized);
        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            x.clone().to_wire_type(),
            __WireX {
                version: 1,
                t: x.clone()
            }
        );
        assert_eq!(x, deserialized);
    }
}
