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

/// Macro that implements [From] trait for 2-way conversion
/// between the orignal type and the target type, using the
/// intermidate type that is convertible from and to both
/// orignal and target types
#[macro_export]
macro_rules! impl_from_with_proxy {
    ($t:ty, $ti:ty, $t2:ty) => {
        impl From<$t> for $t2 {
            fn from(t: $t) -> Self {
                let intermidate: $ti = t.into();
                intermidate.into()
            }
        }

        impl From<$t2> for $t {
            fn from(t: $t2) -> Self {
                let intermidate: $ti = t.into();
                intermidate.into()
            }
        }
    };
}

/// Macro that implements [From] trait for 2-way conversion
/// between the orignal type and the versioned target type, using the
/// intermidate type that is convertible from and to both
/// orignal and target types
#[macro_export]
macro_rules! impl_from_for_versioned_with_proxy {
    ($t:ty, $ti:ty, $t2:ty) => {
        impl From<$t> for $t2 {
            fn from(t: $t) -> Self {
                let intermidate: $ti = t.into();
                intermidate.into()
            }
        }

        impl From<$t2> for $t {
            fn from(t: $t2) -> Self {
                let (intermidate,): ($ti,) = t.into();
                intermidate.into()
            }
        }
    };
}

/// Macro that implements [From] trait for 2-way conversion
/// between the orignal type and the generic target type, using the
/// intermidate type that is convertible from and to both
/// orignal and target types
#[macro_export]
macro_rules! impl_from_for_generic_with_proxy {
    ($t:ty, $ti:ty, $t2:ty) => {
        impl From<$t> for $t2 {
            fn from(t: $t) -> Self {
                let intermidate: $ti = t.into();
                intermidate.into()
            }
        }

        impl From<$t2> for $t {
            fn from(t: $t2) -> Self {
                let (intermidate,): ($ti,) = t.into();
                intermidate.into()
            }
        }
    };
}
