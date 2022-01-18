// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::to_writer;
use std::fmt::Write;

mod common;
use common::print_janestreet_byte_array;

const MAX_BYTES: usize = 11;

const EXPECTED: &str = r#"
.. .. .. .. .. .. .. .. .. .. 00 -> []
.. .. .. .. .. .. .. .. .. 00 01 -> [0]
.. .. .. .. .. .. .. .. 01 00 02 -> [0, 1]
.. .. .. .. .. .. .. ff ff 01 02 -> [1, -1]
.. .. .. .. 7f ff ff ff fd 00 02 -> [0, 2147483647]
80 00 00 00 fd 7f ff ff ff fd 02 -> [2147483647, -2147483648]
"#;

fn test_cases() -> Vec<Vec<i32>> {
    vec![
        vec![],
        vec![0],
        vec![0, 1],
        vec![1, -1],
        vec![0, i32::MAX],
        vec![i32::MAX, i32::MIN],
    ]
}

#[test]
fn test_serialize_arrays() {
    let mut buf = String::new();
    writeln!(&mut buf).unwrap();
    for val in test_cases() {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val).unwrap();
        print_janestreet_byte_array(&mut buf, &output, MAX_BYTES);
        writeln!(&mut buf, "-> {:?}", val).expect("its cooked");
    }

    assert_eq!(&buf, EXPECTED);
}

#[test]
fn test_roundtrip_arrays() {
    for val in test_cases() {
        common::roundtrip_test(val);
    }
}
