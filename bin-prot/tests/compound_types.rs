// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use bin_prot::integers::{integer, nat0};
mod common;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestTupleStruct(bool, i8, i16, i32, i64, (), Option<()>, [u8; 3], char);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct A(bool);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct B {
    a: A,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestFieldAttrs {
    #[serde(with = "nat0")]
    n: u8,
    #[serde(with = "integer")]
    i: i16,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CompressedPoly {
    version: u8,
    x: [u8; 32],
    is_odd: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum E {
    A,
    B,
    C,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PublicKey {
    version: u8,
    poly: CompressedPoly,
}

impl PublicKey {
    pub fn new() -> Self {
        PublicKey {
            version: 1,
            poly: CompressedPoly {
                version: 1,
                x: [0x00; 32],
                is_odd: false,
            },
        }
    }
}

#[test]
fn roundtrip_tuple_struct() {
    common::roundtrip_test(TestTupleStruct(
        true,
        0,
        0,
        0,
        0,
        (),
        None,
        [0x01, 0x02, 0x03],
        'c',
    ));
}

#[test]
fn roundtrip_array_in_struct() {
    common::roundtrip_test(PublicKey::new())
}

#[test]
fn roundtrip_nested_structs() {
    common::roundtrip_test(B { a: A(false) });
}

#[test]
fn roundtrip_enum() {
    common::roundtrip_test(E::A);
}

#[test]
fn roundtrip_owned_string() {
    common::roundtrip_test("serde-bin-prot".to_string());
}
