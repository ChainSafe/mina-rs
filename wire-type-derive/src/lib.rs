// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(default, attributes(wire_type))]
struct Opts {
    version: u8,
}

impl std::default::Default for Opts {
    fn default() -> Self { Opts { version: 1} }
}

#[proc_macro_derive(WireType, attributes(wire_type))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let Opts { version } = Opts::from_derive_input(&input)
        .expect("Invalid options for wire_type. Must provide a version number (e.g. #[wire_type(version = 1)]");
    let DeriveInput { ident, .. } = input;

    let wire_ident = format_ident!("__Wire{}", ident);

    let output = quote! {

        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct #wire_ident {
            version: u8,
            inner: #ident
        }

        impl From<#wire_ident> for #ident {
            fn from(t: #wire_ident) -> Self { t.inner }
        }

        impl From<#ident> for #wire_ident {
            fn from(t: #ident) -> #wire_ident { Self { version: #version, inner: t } }
        }

        impl<'a> wire_type::WireType<'_> for #ident {
            type WireType = #wire_ident;
            const VERSION: u8 =  #version;
            fn to_wire_type(self) -> Self::WireType { Self::WireType { version: Self::VERSION, inner: self } }
            fn from_wire_type(t: Self::WireType) -> Self { Self::from(t) }
        }

    };
    output.into()
}
