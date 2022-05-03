// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use syn::{punctuated::Punctuated, AttrStyle, Attribute, Field, Token};

pub fn parse_types_from_attr(attributes: &[Attribute]) -> Vec<proc_macro2::TokenStream> {
    let mut target_types: Vec<proc_macro2::TokenStream> = Vec::new();
    for attr in attributes {
        if let AttrStyle::Outer = attr.style {
            if let Some(attr_id) = attr.path.get_ident() {
                if &attr_id.to_string() == "auto_from" {
                    if let Ok(args) = attr.parse_args::<proc_macro2::TokenStream>() {
                        target_types.push(args.clone());
                    }
                }
            }
        };
    }
    target_types
}

pub fn auto_from_for_struct_with_named_fields(
    ident: &proc_macro2::Ident,
    target_types: &[proc_macro2::TokenStream],
    named: Punctuated<Field, Token![,]>,
) -> Option<TokenStream> {
    let mut field_idents = Vec::new();
    let mut vec_field_idents = Vec::new();
    let mut option_field_idents = Vec::new();
    'outer: for f in named {
        if let Some(ident) = f.ident {
            if let syn::Type::Path(type_path) = f.ty {
                for seg in type_path.path.segments {
                    match seg.ident.to_string().as_str() {
                        "Vec" => {
                            vec_field_idents.push(ident);
                            continue 'outer;
                        }
                        "Option" => {
                            option_field_idents.push(ident);
                            continue 'outer;
                        }
                        _ => {}
                    };
                }
            }
            field_idents.push(ident);
        }
    }
    if field_idents.is_empty() && vec_field_idents.is_empty() && option_field_idents.is_empty() {
        return None;
    }
    let mut output = TokenStream::default();

    for target_type in target_types {
        let ts: TokenStream = quote! {
            impl ::std::convert::From<#ident> for #target_type {
                fn from(item: #ident) -> Self {
                    Self {
                        #(#field_idents: item.#field_idents.into(),) *
                        #(#vec_field_idents: item.#vec_field_idents.into_iter().map(::std::convert::Into::into).collect(),) *
                        #(#option_field_idents: item.#option_field_idents.map(::std::convert::Into::into),) *
                    }
                }
            }

            impl ::std::convert::From<#target_type> for #ident {
                fn from(item: #target_type) -> Self {
                    Self {
                        #(#field_idents: item.#field_idents.into(),) *
                        #(#vec_field_idents: item.#vec_field_idents.into_iter().map(::std::convert::Into::into).collect(),) *
                        #(#option_field_idents: item.#option_field_idents.map(::std::convert::Into::into),) *
                    }
                }
            }
        }
        .into();
        output.extend(ts);
    }

    Some(output)
}

pub fn auto_from_for_struct_with_unnamed_fields(
    ident: &proc_macro2::Ident,
    target_types: &[proc_macro2::TokenStream],
    named: Punctuated<Field, Token![,]>,
) -> Option<TokenStream> {
    let mut pos_token_stream: Vec<proc_macro2::TokenStream> = Vec::new();
    'outer: for f in named {
        if f.ident.is_none() {
            let pos = proc_macro2::Literal::usize_unsuffixed(pos_token_stream.len());

            if let syn::Type::Path(type_path) = f.ty {
                for seg in type_path.path.segments {
                    match seg.ident.to_string().as_str() {
                        "Vec" => {
                            pos_token_stream.push(
                                quote! {item.#pos.into_iter().map(::std::convert::Into::into).collect()}
                            );
                            continue 'outer;
                        }
                        "Option" => {
                            pos_token_stream
                                .push(quote! {item.#pos.map(::std::convert::Into::into)});
                            continue 'outer;
                        }
                        _ => {}
                    };
                }
            }

            pos_token_stream.push(quote! {item.#pos.into()});
        }
    }
    if pos_token_stream.is_empty() {
        return None;
    }
    let mut output = TokenStream::default();

    for target_type in target_types {
        let ts: TokenStream = quote! {
            impl ::std::convert::From<#ident> for #target_type {
                fn from(item: #ident) -> Self {
                    Self (
                        #(#pos_token_stream,) *
                    )
                }
            }

            impl ::std::convert::From<#target_type> for #ident {
                fn from(item: #target_type) -> Self {
                    Self (
                        #(#pos_token_stream,) *
                    )
                }
            }
        }
        .into();
        output.extend(ts);
    }

    Some(output)
}
