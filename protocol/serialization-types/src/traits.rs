// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This crate provides traits for type conversions
//!

/// This trait provides string conversion methods that match
/// signature of [std::str::FromStr], when these traits
/// cannot be directly implemented for external types. Do not
/// use this trait for internal types
pub trait StrConv: Sized {
    /// Error occured during string conversion
    type Error;

    /// Fn from [std::str::FromStr]
    fn from_str(s: &str) -> Result<Self, Self::Error>;

    /// Avoid naming conflicts with `try_into` in [TryInto]
    fn try_into_string(self) -> Result<String, Self::Error>;
}
