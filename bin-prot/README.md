# Serde Bin-prot

![example workflow](https://github.com/ChainSafe/serde-bin-prot/actions/workflows/build-and-test.yml/badge.svg) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
A Rust crate that adds serde support for the [Bin_prot](https://github.com/janestreet/bin_prot) serialization format

IMPORTANT: This is a work in progress. There is not currently support for all types or full test coverage.

## Usage

Following Serde convention this crate exposes a `from_reader` and `to_writer` function to serialize/deserialize bin_prot encoded streams of bytes.

This works with any Serde supported data type or any custom type annotated with `#[derive(Serialize, Deserialize)]`.

Example:

```rust
use bin_prot::{from_reader, to_writer};

fn main() {
  let val: Vec<i64> = vec![20, -22, 38];

  # Example write into a vec of bytes
  let mut output = Vec::<u8>::new();
  to_writer(&mut output, &val).unwrap();

  # Read back into a typed value
  let de_val: Vec<i64> = from_reader(output.as_slice()).unwrap();
  
  assert!(val == de_val)
}

```

## Testing

All tests can be run through cargo

```
cargo test
```

## Licence

Distributed under the Apache-2.0 License. See LICENSE for more information.

## Contact

[Willem Olding](mailto:willem@chainsafe.io)
