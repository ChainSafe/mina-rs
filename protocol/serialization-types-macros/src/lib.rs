// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This crate provides helper procedural macros for type conversions,
//! it supports struct(s) with named and unnamed fields, enums.
//!
//! # Example
//!
//! ```
//! #[derive(Debug, Clone, PartialEq, derive_more::From, derive_more::Into)]
//! struct I64(pub i64);
//!
//! #[derive(Debug, Clone, PartialEq, mina_serialization_types_macros::AutoFrom)]
//! #[auto_from(Bar)]
//! enum Foo {
//!     V1,
//!     V2(i64),
//!     V3(i64, Option<i64>, Vec<i64>, Box<i64>),
//!     V4 {
//!         f1: i64,
//!     },
//!     V5 {
//!         f1: i64,
//!         f2: Option<i64>,
//!         f3: Vec<i64>,
//!         f4: Box<i64>,
//!     },
//! }

//! #[derive(Debug, Clone, PartialEq)]
//! enum Bar {
//!     V1,
//!     V2(I64),
//!     V3(I64, Option<I64>, Vec<I64>, Box<I64>),
//!     V4 {
//!         f1: I64,
//!     },
//!     V5 {
//!         f1: I64,
//!         f2: Option<I64>,
//!         f3: Vec<I64>,
//!         f4: Box<I64>,
//!     },
//! }

//! type BarV1 = versioned::Versioned<Bar, 1>;
//! let foo = Foo::V5 {
//!     f1: 8,
//!     f2: Some(9),
//!     f3: vec![10, 11, 12],
//!     f4: Box::new(13),
//! };
//!
//! let bar: Bar = foo.clone().into();
//! let bar_v1: BarV1 = foo.clone().into();
//! let foo_from_bar: Foo = bar.into();
//! let foo_from_bar_v1: Foo = bar_v1.into();
//! assert_eq!(foo, foo_from_bar);
//! assert_eq!(foo, foo_from_bar_v1);
//! ```
//!

#![deny(warnings)]
#![deny(missing_docs)]

mod auto_from;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput, FieldsNamed, FieldsUnnamed};

/// A derive macro that automatically implements [From] trait between the annotated type
/// and types including the attributed target type(s) and their versioned types,
/// when the target type has identical field names with the annotated one, and each pair of the fields
/// are convertible between each other
#[proc_macro_derive(AutoFrom, attributes(auto_from))]
pub fn auto_from_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let target_types: Vec<proc_macro2::TokenStream> =
        auto_from::parse_types_from_attr(attrs.as_slice());

    if target_types.is_empty() {
        return Default::default();
    }

    match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                if let Some(ts) = auto_from::auto_from_for_struct_with_named_fields(
                    &ident,
                    target_types.as_slice(),
                    named,
                ) {
                    return ts;
                }
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if let Some(ts) = auto_from::auto_from_for_struct_with_unnamed_fields(
                    &ident,
                    target_types.as_slice(),
                    unnamed,
                ) {
                    return ts;
                }
            }
            _ => {}
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            if let Some(ts) =
                auto_from::auto_from_for_enum(&ident, target_types.as_slice(), variants)
            {
                return ts;
            }
        }
        _ => {}
    };

    Default::default()
}
