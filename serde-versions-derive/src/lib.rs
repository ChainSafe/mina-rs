// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! # Serde Versions Derive
//!
//!  `serde_versions_derive` exports an attribute macro that adds versioning support for structs.
//!  
//!  When serializing a named field struct it will automatically add a new field containing the version.
//!  It also allows deserializing the versioned type directly back to the unversioned one.
//!  
//!  Under the hood this works by creating a new struct that wraps the original struct plus adds a version byte field.
//!  Internally this new struct uses `#[serde(flatten)]` to serialize as expected.
//!  The original struct uses `#[serde(to, from)]` to add the version field when serializing and remove it when deserializing.
//!
//! usage:
//! ```no_run
//! # use serde::{Deserialize, Serialize};
//! # use serde_versions_derive::version;
//! #[version(3)]
//! #[derive(Clone, Serialize, Deserialize)]
//! struct S {
//!     i: i32,
//! }
//! ```
//!
//! This produces the following
//! ```ignore
//! #[derive(Clone, Serialize, Deserialize)]
//! #[serde(into = "_Sv3", from = "_Sv3")]
//! struct S {
//!     i: i32,
//! }
//!
//! #[derive(Clone, Serialize, Deserialize)]
//! struct _Sv3 {
//!     version: u8,
//!     #[serde(flatten)]
//!     inner: S
//! }
//!
//! // plus implementations of To, From and to_versioned() for S
//! ```
//!
//! This supports types with type parameters however these must have a trait bound
//! to implement Clone
//!
//! e.g.:
//! ```no_run
//! # use serde::{Deserialize, Serialize};
//! # use serde_versions_derive::version;
//! #[version(3)]
//! #[derive(Clone, Serialize, Deserialize)]
//! struct S<T: Clone> {
//!     t: T,
//! }
//! ```
//!  

use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::{parse::Parser, parse_macro_input, DeriveInput, LitInt};

/// Generate a new struct with a version field and ensure this struct is converted to that form before
/// serialization.
///
/// See crate doc for example.
///
#[proc_macro_attribute]
pub fn version(attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_ast = parse_macro_input!(item as DeriveInput);

    let mut versioned_ast = original_ast.clone();

    let (impl_generics, generics, _) = original_ast.generics.split_for_impl();
    let version = parse_macro_input!(attr as LitInt);
    let struct_name = original_ast.ident.clone();

    // name is old struct name with V<version_number> appended
    let versioned_name = format_ident!("_{}v{}", original_ast.ident, version.to_string());
    let versioned_name_str = format!(
        "{}{}",
        versioned_name.to_string(),
        quote! {#generics}.to_string()
    );
    versioned_ast.ident = versioned_name.clone();

    match &mut versioned_ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                // for named field structs e.g. { A: int }
                syn::Fields::Named(fields) => {
                    // used to convert between unversioned and versioned
                    let mut field_mapping = quote!();
                    let mut field_mapping_back = quote!();
                    for field in fields.named.iter() {
                        let name = field.ident.as_ref().unwrap();
                        field_mapping.extend(quote!(
                            #name : self . #name,
                        ));
                        field_mapping_back.extend(quote!(
                            #name : s . #name,
                        ));
                    }

                    fields.named.insert(
                        0,
                        syn::Field::parse_named
                            .parse2(quote! { version: u8 })
                            .unwrap(),
                    );

                    (quote! {
                        #[serde(into = #versioned_name_str, from = #versioned_name_str)]
                        #original_ast

                        #versioned_ast

                        impl #impl_generics #struct_name #generics {
                            pub fn into_versioned(self) -> #versioned_name #generics {
                                #versioned_name {
                                    version: #version,
                                    #field_mapping
                                }
                            }
                        }

                        impl #impl_generics std::convert::From<#struct_name #generics> for #versioned_name #generics {
                            fn from(s: #struct_name #generics) -> #versioned_name #generics {
                                s.into_versioned()
                            }
                        }

                        impl #impl_generics std::convert::From<#versioned_name #generics> for #struct_name #generics {
                            fn from(s: #versioned_name #generics) -> #struct_name #generics {
                                #struct_name {
                                    #field_mapping_back
                                }
                            }
                        }
                    })
                    .into()
                }
                // for unnamed fields e.g. A(int)
                syn::Fields::Unnamed(fields) => {
                    // used to convert between unversioned and versioned
                    let mut field_mapping = quote!();
                    let mut field_mapping_back = quote!();
                    for (i, _) in fields.unnamed.iter().enumerate() {
                        let index = syn::Index::from(i);
                        let versioned_index = syn::Index::from(i + 1);
                        field_mapping.extend(quote!(
                            self . #index,
                        ));
                        field_mapping_back.extend(quote!(
                            s . #versioned_index,
                        ));
                    }

                    fields
                        .unnamed
                        .insert(0, syn::Field::parse_unnamed.parse2(quote! { u8 }).unwrap());

                    (quote! {
                        #[serde(into = #versioned_name_str, from = #versioned_name_str)]
                        #original_ast

                        #versioned_ast

                        impl #impl_generics #struct_name #generics {
                            pub fn into_versioned(self) -> #versioned_name #generics {
                                #versioned_name (
                                    #version,
                                    #field_mapping
                                )
                            }
                        }

                        impl #impl_generics std::convert::From<#struct_name #generics> for #versioned_name #generics {
                            fn from(s: #struct_name #generics) -> #versioned_name #generics {
                                s.into_versioned()
                            }
                        }

                        impl #impl_generics std::convert::From<#versioned_name #generics> for #struct_name #generics {
                            fn from(s: #versioned_name #generics) -> #struct_name #generics {
                                #struct_name (
                                    #field_mapping_back
                                )
                            }
                        }
                    })
                    .into()
                }
                // for unit types e.g. A()
                syn::Fields::Unit => {
                    unimplemented!()
                }
            }
        }
        _ => panic!("`version` has to be used with structs "),
    }
}
