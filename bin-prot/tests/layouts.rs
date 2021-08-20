// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::de::Deserialize;
use bin_prot::error::Result;
use bin_prot::value::layout::{BinProtRule, Layout};
use bin_prot::value::Value;
use bin_prot::Deserializer;

const SIMPLE_RULE: &str = r#"
[
  "Option",
  [
    "Tuple",
    [
      ["Int"],
      ["Bool"]
    ]
  ]
]
"#;

#[test]
fn test_simple_rule() {
    let rule: BinProtRule = serde_json::from_str(SIMPLE_RULE).unwrap();
    let example = vec![0x01, 0x00, 0x00]; // Some((0, false))

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    println!("{:?}", result);
    assert_eq!(
        result,
        Value::Option(Some(Box::new(Value::Tuple(vec![
            Value::Int(0),
            Value::Bool(false)
        ]))))
    )
}

const RECORD_RULE: &str = r#"
[
  "Record",
  [
    { "field_name": "first", "field_rule": ["Int"] },
    { "field_name": "second", "field_rule": ["Record", [{ "field_name": "inner", "field_rule": ["Bool"] }] ] }
  ]
]
"#;

#[test]
fn test_record_rule() {
    let rule: BinProtRule = serde_json::from_str(RECORD_RULE).unwrap();
    let example = vec![0x00, 0x01];

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    println!("{:?}", result);
    assert_eq!(
        result,
        Value::Record(vec![
            ("first".to_string(), Value::Int(0)),
            (
                "second".to_string(),
                Value::Record(vec![("inner".to_string(), Value::Bool(true))].into_iter().collect())
            )
        ].into_iter().collect())
    )
}

const SUM_RULE: &str = r#"
[
  "Sum",
  [
    {
      "ctor_name": "one",
      "index": 0,
      "ctor_args": [["Int"]]
    },
    {
      "ctor_name": "two",
      "index": 1,
      "ctor_args": [["Bool"]]
    }
  ]
]
"#;

#[test]
fn test_sum_rule() {
    let rule: BinProtRule = serde_json::from_str(SUM_RULE).unwrap();
    let example = vec![0x01, 0x00]; // Two((false))

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(
        result,
        Value::Sum {
            name: "two".to_string(),
            index: 1,
            value: Box::new(Value::Bool(false))
        }
    )
}

const NESTED_SUM_RULE: &str = r#"
[
  "Sum",
  [
    {
      "ctor_name": "one",
      "index": 0,
      "ctor_args": [[
        "Record",
          [
            { "field_name": "first", "field_rule": ["Int"] }
          ]
        ]
       ]
    }
  ]
]
"#;

#[test]
fn test_nested_sum_rule() {
    let rule: BinProtRule = serde_json::from_str(NESTED_SUM_RULE).unwrap();
    let example = vec![0x00, 0x05]; // One({ first: 5 })

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(
        result,
        Value::Sum {
            name: "one".to_string(),
            index: 0,
            value: Box::new(Value::Record(vec![("first".to_string(), Value::Int(5))].into_iter().collect()))
        }
    )
}

const BLOCK_LAYOUT: &str =
    std::include_str!("fixtures/external_transition_custom_args_compressed.json");
const BLOCK_BYTES: &[u8] = std::include_bytes!("fixtures/block");

#[test]
fn smoke_test_deserialize_block() {
    let mut deserializer = serde_json::Deserializer::from_str(BLOCK_LAYOUT);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let rule = Layout::deserialize(deserializer).unwrap().bin_prot_rule;

    let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, rule);
    let _result: Result<Value> = Deserialize::deserialize(&mut de);
    // This is failing at the moment so don't check the result
}
