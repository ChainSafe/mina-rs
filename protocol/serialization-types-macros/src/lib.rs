// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

mod auto_from;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

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
                if let Some(ts) =
                    auto_from::auto_from_for_struct(&ident, target_types.as_slice(), named)
                {
                    return ts;
                }
            }
            syn::Fields::Unnamed(_) => {
                unimplemented!();
            }
            _ => {}
        },
        syn::Data::Enum(_) => {
            unimplemented!();
        }
        _ => {}
    };

    Default::default()
}
