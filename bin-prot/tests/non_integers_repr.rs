mod common;

use crate::common::{BInner, BInner2, BInner3, BInner4, CInner, CInner2, A, B, B2, B3, C};
use std::vec as array;

// Non-integer bin_prot size tests
// Ported from: https://github.com/janestreet/bin_prot/blob/master/test/non_integers_repr.ml

#[test]
fn test_unit() {
    bin_prot_test! {
        0x00 -> ()
    }
}

#[test]
fn test_bool() {
    bin_prot_test! {
        0x01 -> true,
        0x00 -> false
    }
}

#[test]
fn test_char() {
    bin_prot_test! {
        0x00 -> '\u{000}',
        0x41 -> 'A',
        0x7a -> 'z',
        0x3b -> ';',
        0xff -> 255u8 as char
    }
}

#[test]
fn test_float() {
    bin_prot_test! {
        0x3c 0xb0 00 00 00 00 00 00 -> 2.2204460492503131E-16,
        0x7f 0xf0 00 00 00 00 00 00 -> f64::INFINITY,
        0x7f 0xef 0xff 0xff 0xff 0xff 0xff 0xff -> 1.7976931348623157E+308,
        0x7f 0xf0 00 00 00 00 00 00 -> f64::INFINITY,
        00 0x10 00 00 00 00 00 00 -> 2.2250738585072014E-308,
        00 00 00 00 00 00 00 0x01 -> 4.94065645841247E-324,
        0xff 0xf0 00 00 00 00 00 00 -> f64::NEG_INFINITY,
        0xbf 0xf0 00 00 00 00 00 00 -> -1f64,
        0xff 0xf0 00 00 00 00 00 00 -> f64::NEG_INFINITY,
        0x3f 0xf0 00 00 00 00 00 00 -> 1f64,
        0x3e 0x7a 0xd7 0xf2 0x9a 0xbc 0xaf 0x48 -> 1E-07
        // 00 00 00 00 00 00 00 00 -> 0 // FIXME
    }
}

#[test]
fn test_vec() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. 0x00 -> Vec::<i32>::new()
        // 0x3f 0xf0 0x00 0x00 0x00 0x00 0x00 0x00 0x01 -> vec![1i32] // FIXME: see: https://github.com/janestreet/bin_prot/blob/5915cde59105f398b53f682c5f4dad29e272f696/test/non_integers_repr.ml#L625
    }
}

#[test]
fn test_float32_vec() {
    bin_prot_test! {
        .. .. .. .. 0x00 -> Vec::<f32>::new(),
        0x3f 0x80 0x00 0x00 0x01 -> vec![1f32]
    }
}

#[test]
fn test_float64_vec() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. 0x00 -> Vec::<f64>::new(),
        0x3f 0xf0 0x00 0x00 0x00 0x00 0x00 0x00 0x01 -> vec![1f64]
    }
}

#[test]
fn test_option() {
    bin_prot_test! {
        .. .. .. .. .. 0x00 -> Option::<i64>::None,
        .. .. .. .. 0x00 0x01 -> Some(0i64),
        .. .. .. .. 0x01 0x01 -> Some(1i64),
        .. .. .. 0xff 0xff 0x01 -> Some(-1),
        0x7f 0xff 0xff 0xff 0xfd 0x01 -> Some(2147483647i64),
        0x80 0x00 0x00 0x00 0xfd 0x01 -> Some(-2147483648i64)
    }
}

#[test]
fn test_pair() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. 0x00 0x00 -> ((0, 0)),
        .. .. .. .. .. .. .. .. 0x01 0x01 -> ((1, 1)),
        .. .. .. .. .. .. 0xff 0xff 0xff 0xff -> ((-1, -1)),
        0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xfd -> ((2147483647, 2147483647)),
        0x80 0x00 0x00 0x00 0xfd 0x80 0x00 0x00 0x00 0xfd -> ((-2147483648, -2147483648))
    }
}

#[test]
fn test_triple() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. .. .. 0x00 0x00 0x00 -> ((0, 0, 0)),
        .. .. .. .. .. .. .. .. .. .. .. .. 0x01 0x01 0x01 -> ((1, 1, 1)),
        .. .. .. .. .. .. .. .. .. 0xff 0xff 0xff 0xff 0xff 0xff -> ((-1, -1, -1)),
        0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xfd -> ((2147483647, 2147483647, 2147483647)),
        0x80 0x00 0x00 0x00 0xfd 0x80 0x00 0x00 0x00 0xfd 0x80 0x00 0x00 0x00 0xfd -> ((-2147483648, -2147483648, -2147483648))
    }
}

#[test]
fn test_list() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. 0x00 -> (Vec::<i32>::new()),
        .. .. .. .. .. .. .. .. .. 0x00 0x01 -> (vec![0i32;1]),
        .. .. .. .. .. .. .. .. 0x01 0x00 0x02 -> (vec![0i32, 1]),
        .. .. .. .. .. .. .. 0xff 0xff 0x01 0x02 -> (vec![1i32, -1]),
        .. .. .. .. 0x7f 0xff 0xff 0xff 0xfd 0x00 0x02 -> (vec![0i32, 2147483647]),
        0x80 0x00 0x00 0x00 0xfd 0x7f 0xff 0xff 0xff 0xfd 0x02 -> (vec![2147483647i32, -2147483648])
    }
}

#[test]
fn test_array() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. 0x00 -> (bin_prot::OcamlArray::<i32>::new()),
        .. .. .. .. .. .. .. .. .. 0x00 0x01 -> (array![0i32;1]),
        .. .. .. .. .. .. .. .. 0x01 0x00 0x02 -> (array![0i32, 1]),
        .. .. .. .. .. .. .. 0xff 0xff 0x01 0x02 -> (array![1i32, -1]),
        .. .. .. .. 0x7f 0xff 0xff 0xff 0xfd 0x00 0x02 -> (array![0i32, 2147483647]),
        0x80 0x00 0x00 0x00 0xfd 0x7f 0xff 0xff 0xff 0xfd 0x02 -> (array![2147483647i32, -2147483648])
    }
}

#[test]
fn test_record1() {
    bin_prot_test! {
        .. .. .. .. 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00 -> A {x:0,y:0.0},
        0x7f 0xf0 0x00 0x00 0x00 0x00 0x00 0x00 0x7f 0xff 0xff 0xff 0xfd -> A {x:2147483647,y:f64::INFINITY}
    }
}

#[test]
fn test_record2() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. .. .. 0x00 0x00 0x00 -> B {
            y: BInner { w: 0, x: 0 },
            z: None,
        },
        0x00 0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xfc -> B {
            y: BInner { w: 9223372036854775807, x: 2147483647 },
            z: None,
        }
    }
}

#[test]
fn test_inline_record() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. .. .. 0x00 0x00 0x00 0x00 0x01-> B3 {
            y: BInner2::V1(BInner4::V0(BInner { w: 0, x: 0 })),
            z: None,
        },
        0x00 0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xff 0xff 0xff 0xff 0xfc 0x00 0x01 -> B2 {
            y: BInner3 { dummy: 1 , inner: BInner2::V0(BInner { w: 9223372036854775807, x: 2147483647 })},
            z: None,
        },
        0x00 0x00 0x01 0x01 -> C {
            y: CInner::V1(CInner2::V1(())),
            z: None,
        }
    }
}

// Unimplemented tests
