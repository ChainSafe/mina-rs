# Serde Bin-prot

![example workflow](https://github.com/ChainSafe/serde-bin-prot/actions/workflows/build-and-test.yml/badge.svg) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
A Rust crate that adds serde support for the [Bin_prot](https://github.com/janestreet/bin_prot) serialization format

IMPORTANT: This is a work in progress. There is not currently support for all types or full test coverage.

## Usage

Following Serde convention this crate exposes a `from_reader` and `to_writer` function to serialize/deserialize bin_prot encoded streams of bytes.

### Strongly Typed

This works with any Serde supported data type or any custom type annotated with `#[derive(Serialize, Deserialize)]`.

Example:

```rust
use bin_prot::{from_reader, to_writer};

fn main() {
  let val: Vec<i64> = vec![20, -22, 38];

  // Example write into a vec of bytes
  let mut output = Vec::<u8>::new();
  to_writer(&mut output, &val).unwrap();

  // Read back into a typed value
  let de_val: Vec<i64> = from_reader(output.as_slice()).unwrap();

  assert!(val == de_val)
}

```

### Loosely Typed

Despite bin_prot being a non-self-describing format it is possible to deserialize into a loosely typed value if a layout descriptor file is provided. The layout files are typically written in JSON and describe the nested data structure that
is to be deserialized. For the specification of the BinProtRule type see src/value/layout/mod.rs.

Note that, Loosely-typed deserialization is behind feature `loose_deserialization` which is disabled by default. Use below config snippet to turn it on.

```toml
# in Cargo.toml
bin-prot = {version="$version", features = ["loose_deserialization"]}
```

These are created using the `Deserializer::from_reader_with_layout` constructor.

```rust
use serde::Deserialize;
use bin_prot::{Deserializer, BinProtRule, Value};

// this rule describes a record with two fields, one itself being a record with a bool field
const RECORD_RULE: &str = r#"
[
  "Record",
  [
    { "field_name": "first", "field_rule": ["Int"] },
    { "field_name": "second", "field_rule": ["Record", [{ "field_name": "inner", "field_rule": ["Bool"] }] ] }
  ]
]
"#;

fn main() {
    let rule: BinProtRule = serde_json::from_str(RECORD_RULE).unwrap();

    // this is the compact encoding of { first: 0, second: { inner: true } }
    let bytes = vec![0x00, 0x01];

    // provide both the raw bytes and the rule descriptor to `from_reader_with_layout`
    let mut de = Deserializer::from_reader(bytes.as_slice()).with_layout(&rule);

    // we can now deserialize into a bin_prot::Value type!
    let result: Value = Deserialize::deserialize(&mut de).unwrap();

    // These support indexing (strings for struct fields and usize for tuple/vector types)
    assert_eq!(
        result["second"]["inner"],
        Value::Bool(true)
    )
}

```

## Testing

All tests can be run through cargo

```shell
cargo test
```

## Licence

Distributed under the Apache-2.0 License. See LICENSE for more information.

## Contact

[Willem Olding](mailto:willem@chainsafe.io)
