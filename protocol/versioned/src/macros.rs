// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Macros that help implement common traits for versioned wrapper types
//!

/// Macro that implements [From] trait for versioned wrapper
#[macro_export]
macro_rules! impl_from_for_newtype {
    ($t:ty, $tv:ty) => {
        impl From<$t> for $tv {
            fn from(t: $t) -> Self {
                t.0.into()
            }
        }

        impl From<$tv> for $t {
            fn from(t: $tv) -> Self {
                let (t,) = t.into();
                Self(t)
            }
        }
    };
}

/// Macro that implements [From] trait for the extension type
/// that is convertible from and to the versioned type
#[macro_export]
macro_rules! impl_from_for_ext_type {
    ($t:ty, $tv:ty, $t2:ty) => {
        impl From<$t> for $t2 {
            fn from(t: $t) -> Self {
                let versioned: $tv = t.into();
                versioned.into()
            }
        }

        impl From<$t2> for $t {
            fn from(t: $t2) -> Self {
                let versioned: $tv = t.into();
                versioned.into()
            }
        }
    };
}

/// Macro that implements [From] trait for the generic extension type
/// that is convertible from and to the versioned type
#[macro_export]
macro_rules! impl_from_for_ext_type_generic {
    ($t:ty, $tv:ty, $t2:ty) => {
        impl From<$t> for $t2 {
            fn from(t: $t) -> Self {
                let versioned: $tv = t.into();
                versioned.into()
            }
        }

        impl From<$t2> for $t {
            fn from(t: $t2) -> Self {
                let (versioned,): ($tv,) = t.into();
                versioned.into()
            }
        }
    };
}
