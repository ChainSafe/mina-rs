// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::error::Error;
use bin_prot::Deserializer;
use serde::{Deserialize, Serialize};

mod common;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "Polyvar")]
enum TestPolyvar {
    #[serde(rename = "None")] // Hash repr 870530776_u32
    VariantNone,
    #[serde(rename = "One")] // Hash repr 3953222_u32
    VariantOne(bool),
    #[serde(rename = "Two")] // Hash repr 4203884_u32
    VariantTwo(TestPolyvar2),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "Polyvar")]
enum TestPolyvar2 {
    #[serde(rename = "Some")] // Hash repr 925978388_u32
    SomeVariant,
}

#[test]
fn test_polyvar_variant_none() {
    let tag = 870530776_u32.to_le_bytes();
    let mut de = Deserializer::from_reader(tag.as_slice());
    let result: TestPolyvar = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(result, TestPolyvar::VariantNone)
}

#[test]
fn test_polyvar_variant_one() {
    let tag = 3953222_u32.to_le_bytes();
    let value = 0x01_u32.to_le_bytes();
    let data: Vec<u8> = [tag, value].concat();

    let mut de = Deserializer::from_reader(data.as_slice());
    let result: TestPolyvar = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(result, TestPolyvar::VariantOne(true))
}

#[test]
fn test_polyvar_variant_two() {
    let tag = 4203884_u32.to_le_bytes();
    let value = 925978388_u32.to_le_bytes();
    let data: Vec<u8> = [tag, value].concat();

    let mut de = Deserializer::from_reader(data.as_slice());
    let result: TestPolyvar = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(result, TestPolyvar::VariantTwo(TestPolyvar2::SomeVariant))
}

#[test]
fn test_polyvar_unknown_polyvar_tag() {
    let tag = 1234567_u32.to_le_bytes(); // random hash
    let mut de = Deserializer::from_reader(tag.as_slice());
    let result: Result<TestPolyvar, Error> = Deserialize::deserialize(&mut de);
    assert!(result.is_err())
}
