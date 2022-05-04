// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Heper macros for type conversions
//!

/// Implements [std::str::FromStr] and [std::fmt::Display] by implementing
/// [TryFrom] between given type and string types via its corresponding
/// json serialization type which is convertible from / to json with single
/// unnamed string field.
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
                h.clone().try_into()
            }
        }

        impl ::std::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                let s: String = self.try_into().map_err(|_| ::std::fmt::Error::default())?;
                write!(f, "{s}")
            }
        }
    };
}
