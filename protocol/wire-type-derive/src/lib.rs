// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]
#![deny(missing_docs)]

//!
//! This crate contains procedural macro for for deriving WireType trait
//!

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[derive(FromDeriveInput)]
#[darling(default, attributes(wire_type))]
struct Opts {
    version: u16,
    recurse: usize,
}

impl std::default::Default for Opts {
    fn default() -> Self {
        Opts {
            version: 1,
            recurse: 1,
        }
    }
}

#[proc_macro_derive(WireType, attributes(wire_type))]
///
/// Derive macro for WireType trait
///
/// This creates a new struct for the type that is annotated wraps the original type in a field `t` and adds a `version` field.
/// The WireType trait implementation is generated such that this new struct is the wire type.
/// This aims to produce types that are have serde serialization compatible with the types serialized from the Mina OCaml implementation.
///
/// By default the version is 1, this can be changed by adding the attribute `#[wire_type( version = X )]`
///
/// It is also possible to produce a multiple nested wire type e.g. `{"version":1,"t":{"version":1,"t":{"a":123,"b":321}}}`
/// by adding the attribute `#[wire_type( recurse = 2 )]`
///
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let Opts { version, recurse } = Opts::from_derive_input(&input)
        .expect("Invalid options for wire_type. Must provide a version number (e.g. #[wire_type(version = 1)]");
    let shadow = shadow_from_input(input.clone());
    let DeriveInput {
        ident,
        generics: _,
        data: _,
        ..
    } = input;

    let wire_ident = format_ident!("__Wire{}", ident);
    let shadow_ident = format_ident!("__Shadow{}", ident);
    let shadow_ident_str = shadow_ident.to_string();

    // this is a recursive way to allow nested versioning
    // e.g. `{"version":1,"t":{"version":1,"t":{"a":123,"b":321}}}`
    // which occurs sometimes in the Mina codebase
    // This may need to be rewritten if it doesn't work in all cases
    let recurse_attr = if recurse > 1 {
        let recurse_less_1 = recurse - 1;
        Some(quote! {
            #[derive(Clone, WireType)]
            #[wire_type( recurse = #recurse_less_1 )]
            #[serde(from = "<Self as WireType>::WireType")]
            #[serde(into = "<Self as WireType>::WireType")]
        })
    } else {
        None
    };

    let output = quote! {

        #[allow(missing_docs)]
        #shadow

        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        #recurse_attr
        pub struct #wire_ident {
            version: u16,
            #[serde(with = #shadow_ident_str)]
            t: #ident
        }

        #[automatically_derived]
        impl From<#wire_ident> for #ident {
            fn from(t: #wire_ident) -> Self { t.t }
        }

        #[automatically_derived]
        impl From<#ident> for #wire_ident {
            fn from(t: #ident) -> #wire_ident { Self { version: #version, t: t } }
        }

        #[automatically_derived]
        impl<'a> wire_type::WireType<'_> for #ident {
            type WireType = #wire_ident;
            const VERSION: u16 =  #version;
            fn to_wire_type(self) -> Self::WireType { Self::WireType { version: Self::VERSION, t: self } }
            fn from_wire_type(t: Self::WireType) -> Self { Self::from(t) }
        }

    };
    output.into()
}

// Creates the token stream for a shadow struct/enum for the input
// The shadow struct has an identical structure but its own serialize/deserialize
// implementations. This is to prevent the infinite recursion if using serde from and into
fn shadow_from_input(input: DeriveInput) -> TokenStream2 {
    let ident_str = input.ident.to_string();
    let shadow_ident = format_ident!("__Shadow{}", input.ident);

    let attrs = quote! {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(remote = #ident_str)]
        #[allow(non_camel_case_types)]
    };

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let fields = fields.named;
            quote! {
                #attrs
                struct #shadow_ident { #fields }
            }
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => {
            let fields = fields.unnamed;
            quote! {
                #attrs
                struct #shadow_ident ( #fields );
            }
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => quote! {
            #attrs
            struct #shadow_ident;
        },
        Data::Enum(DataEnum { variants, .. }) => {
            quote! {
                #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
                #[serde(remote = #ident_str)]
                enum #shadow_ident {
                    #variants
                }
            }
        }
        _ => panic!("this derive macro only works on structs and enums (not unions)"),
    }
}
