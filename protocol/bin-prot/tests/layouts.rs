// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "loose_deserialization")]
mod tests {
    use bin_prot::value::layout::BinProtRule;
    use bin_prot::value::Value;
    use bin_prot::Deserializer;
    use serde::{Deserialize, Serialize};
    use std::io::Cursor;

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

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");

        assert_eq!(
            result,
            Value::Option(Some(Box::new(Value::Tuple(vec![
                Value::Int(0),
                Value::Bool(false)
            ]))))
        );
        test_reserialize(&result, &example);
    }

    const RECORD_RULE: &str = r#"
[
  "Record",
  [
    { "field_name": "first", "field_rule": ["Int"] },
    { "field_name": "second", "field_rule": ["Record", [{ "field_name": "inner", "field_rule": ["Bool"] }] ] },
    { "field_name": "third", "field_rule": ["Bool"] }
  ]
]
"#;

    #[test]
    fn test_record_rule() {
        let rule: BinProtRule = serde_json::from_str(RECORD_RULE).unwrap();
        let example = vec![0x05, 0x00, 0x01];

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");

        assert_eq!(
            result,
            Value::Record(vec![
                ("first".to_string(), Value::Int(5)),
                (
                    "second".to_string(),
                    Value::Record(vec![("inner".to_string(), Value::Bool(false))])
                ),
                ("third".to_string(), Value::Bool(true)),
            ])
        );

        // also test using the indexing
        assert_eq!(result["second"]["inner"], Value::Bool(false));
        test_reserialize(&result, &example);
    }

    #[test]
    fn test_record_rule_partial() {
        let rule: BinProtRule = serde_json::from_str(RECORD_RULE).unwrap();
        let example = vec![0x05, 0x00, 0x01];

        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct PartialType {
            first: u8,
            second: Value,
            third: bool,
        }

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: PartialType = Deserialize::deserialize(&mut de).expect("Failed to deserialize");

        assert_eq!(
            result,
            PartialType {
                first: 5,
                second: Value::Record(vec![("inner".to_string(), Value::Bool(false))]),
                third: true
            }
        );

        test_reserialize(&result, &example);
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

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        assert_eq!(
            result,
            Value::Sum {
                name: "two".to_string(),
                index: 1,
                value: Box::new(Value::Tuple(vec![Value::Bool(false)]))
            }
        );
        test_reserialize(&result, &example);
    }

    const TAGGED_POLYVAR_RULE: &str = r#"
[
  "Polyvar",
   [
    [
     "Tagged",
     {
      "polyvar_name": "One",
      "hash": 3953222,
      "polyvar_args": [["Int"]]
     }
    ],
    [
     "Tagged",
     {
      "polyvar_name": "Two",
      "hash": 4203884,
      "polyvar_args": [["Bool"]]
     }
    ]
   ]
]
"#;

    #[test]
    fn test_tagged_polyvar_rule() {
        let rule: BinProtRule = serde_json::from_str(TAGGED_POLYVAR_RULE).unwrap();
        let example = vec![0xd9, 0x4a, 0x80, 0x00, 0x01]; // Two((true))

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        assert_eq!(
            result,
            Value::Polyvar {
                name: "Two".to_string(),
                tag: 4203884,
                value: Box::new(Value::Tuple(vec![Value::Bool(true)]))
            }
        );
        test_reserialize(&result, &example);
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

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        assert_eq!(
            result,
            Value::Sum {
                name: "one".to_string(),
                index: 0,
                value: Box::new(Value::Tuple(vec![Value::Record(vec![(
                    "first".to_string(),
                    Value::Int(5)
                )])]))
            }
        );
        test_reserialize(&result, &example);
    }

    const OPTION_RULE: &str = r#"
[
  "Option",
  ["Int"]
]
"#;

    #[test]
    fn test_option_rule() {
        let rule: BinProtRule = serde_json::from_str(OPTION_RULE).unwrap();

        let example_none = vec![0x00]; // None

        let mut de =
            Deserializer::from_reader(Cursor::new(example_none.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        println!("{:?}", result);
        assert_eq!(result, Value::Option(None));

        test_reserialize(&result, &example_none);

        let example_some = vec![0x01, 0x07]; // Some(7)

        let mut de =
            Deserializer::from_reader(Cursor::new(example_some.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");

        assert_eq!(result, Value::Option(Some(Box::new(Value::Int(0x07)))));
        test_reserialize(&result, &example_some);
    }

    const MULTIPLE_CTOR_ARG_SUM_RULE: &str = r#"
[
  "Sum",
  [
    {
      "ctor_name": "one",
      "index": 0,
      "ctor_args": [
        [
          "Record",
          [
            { "field_name": "first", "field_rule": ["Int"] }
          ]
        ],
        [
          "Record",
          [
            { "field_name": "second", "field_rule": ["Int"] }
          ]
        ]
      ]
    }
  ]
]
"#;

    #[test]
    fn test_multiple_ctor_arg_sum_rule() {
        let rule: BinProtRule = serde_json::from_str(MULTIPLE_CTOR_ARG_SUM_RULE).unwrap();
        let example = vec![0x00, 0x05, 0x06]; // One({ first: 5 }, { second: 6})

        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        assert_eq!(
            result,
            Value::Sum {
                name: "one".to_string(),
                index: 0,
                value: Box::new(Value::Tuple(vec![
                    Value::Record(vec![("first".to_string(), Value::Int(5))]),
                    Value::Record(vec![("second".to_string(), Value::Int(6))])
                ]))
            }
        );
        test_reserialize(&result, &example);
    }

    #[test]
    fn test_extended_u32_rule() {
        let rule: BinProtRule =
            serde_json::from_str(include_str!("layouts/extended_uint32_rule.json")).unwrap();
        let example = vec![0x01, 0x01, 0xff, 0xff];
        let mut de = Deserializer::from_reader(Cursor::new(example.as_slice())).with_layout(&rule);
        let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
        test_reserialize(&result, &example);
        let mut output = vec![];
        let value = &result["t"]["t"];
        bin_prot::to_writer(&mut output, value).expect("Failed writing bin-prot encoded data");
        assert_eq!(output, [0xff, 0xff]);
        assert_eq!(
            value,
            &Value::Int(-1),
            "Integer expected, but found {:#?}",
            value
        );
    }

    pub fn test_reserialize<T>(val: &T, bytes: &[u8])
    where
        T: Serialize,
    {
        let mut output = vec![];
        bin_prot::to_writer(&mut output, val).expect("Failed writing bin-prot encoded data");
        assert_eq!(bytes, output)
    }
}
