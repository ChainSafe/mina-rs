// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]
#![deny(rustdoc::all)]
#![allow(rustdoc::private_doc_tests)]

//!
//! Contains definitions of data structures and APIs for mina consensus
//!

pub mod common;
pub mod error;
pub mod genesis;

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
