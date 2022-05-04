// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Heper macros for type conversions
//!

/// Implement TryFrom between given type and string types
/// via its corresponding json serialization type which is
/// convertible from / to json with single unnamed string field
#[macro_export]
macro_rules! impl_strconv_via_json {
    ($ty:ty, $ty_json:ty) => {
        impl TryFrom<&str> for $ty {
            type Error = serde_json::error::Error;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                let json_string = serde_json::to_string(s)?;
                let json: $ty_json = serde_json::from_str(&json_string)?;
                Ok(json.into())
            }
        }

        impl TryFrom<String> for $ty {
            type Error = serde_json::error::Error;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                s.as_str().try_into()
            }
        }

        impl ::std::str::FromStr for $ty {
            type Err = serde_json::error::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.try_into()
            }
        }

        impl TryFrom<$ty> for String {
            type Error = serde_json::error::Error;
            fn try_from(h: $ty) -> Result<Self, Self::Error> {
                let h: $ty_json = h.into();
                let json_string = serde_json::to_string(&h)?;
                let json: String = serde_json::from_str(&json_string)?;
                Ok(json)
            }
        }

        impl TryFrom<&$ty> for String {
            type Error = serde_json::error::Error;
            fn try_from(h: &$ty) -> Result<Self, Self::Error> {
                let h: $ty_json = h.clone().into();
                let json_string = serde_json::to_string(&h)?;
                let json: String = serde_json::from_str(&json_string)?;
                Ok(json)
            }
        }
    };
}
