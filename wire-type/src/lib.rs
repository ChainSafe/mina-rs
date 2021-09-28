// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub use wire_type_derive::*;

pub type Version = u16;

pub trait WireType<'a>:
    Debug + PartialEq + Serialize + Deserialize<'a> + From<Self::WireType>
{
    type WireType: Serialize + Deserialize<'a>;
    const VERSION: Version;

    fn to_wire_type(self) -> Self::WireType;
    fn from_wire_type(t: Self::WireType) -> Self;
}

#[cfg(test)]
mod tests {
    use crate as wire_type;
    use wire_type::{WireType, Version};
    use serde::{Deserialize, Serialize};

    #[test]
    fn smoke() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct X {
            a: u32,
        }

        #[derive(Serialize, Deserialize)]
        struct WireX {
            version: Version,
            inner: X,
        }

        impl From<WireX> for X {
            fn from(t: WireX) -> Self {
                t.inner
            }
        }

        impl<'a> WireType<'_> for X {
            type WireType = WireX;
            const VERSION: Version = 0x01;
            fn to_wire_type(self) -> Self::WireType {
                Self::WireType {
                    version: Self::VERSION,
                    inner: self,
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
                inner: x.clone()
            }
        );
        assert_eq!(
            X::from_wire_type(WireX {
                version: 1,
                inner: x.clone()
            }),
            x
        )
    }

    #[test]
    fn compatible_with_serde_from_and_into_named_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X {
            a: u32,
            b: u32,
        }

        let x = X {a: 123, b: 321};

        let serialized = serde_json::to_string(&x).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }

    #[test]
    fn compatible_with_serde_from_and_into_unnamed_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X(u32, u32);

        let x = X (123, 321);

        let serialized = serde_json::to_string(&x).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }

    #[test]
    fn compatible_with_serde_from_and_into_unit_struct() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")] // can be deserialized from its wire type
        #[serde(into = "<Self as WireType>::WireType")] // will be serialized to its wire type
        struct X;

        let x = X;

        let serialized = serde_json::to_string(&x).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
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
        println!("serialized = {}", serialized);

        let deserialized: X = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
}
