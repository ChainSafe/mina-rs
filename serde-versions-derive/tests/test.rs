// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![allow(legacy_derive_helpers)]

use serde::{Deserialize, Serialize};
use serde_versions_derive::version;

#[version(3)]
#[derive(Clone, Serialize, Deserialize)]
struct S {
    i: i32,
    b: bool,
    o: Option<u8>,
}

#[version(33)]
#[derive(Clone, Serialize, Deserialize)]
struct SS<T: Clone>(T);

#[test]
fn to_versioned() {
    let s = S {
        i: 0,
        b: true,
        o: Some(8),
    };
    assert_eq!(s.into_versioned().version, 3);
}

#[test]
fn to_json_has_version() {
    let s = S {
        i: 0,
        b: true,
        o: Some(8),
    };
    let json_str_s = serde_json::to_string(&s).unwrap();
    println!("{}", json_str_s); // version was added when serializing
    let json_s: serde_json::Value = serde_json::from_str(&json_str_s).unwrap();
    assert_eq!(json_s["version"], 3);
}

#[test]
fn to_json_has_version_unnamed() {
    let s = SS(123);
    let json_str_s = serde_json::to_string(&s).unwrap();
    println!("{}", json_str_s); // version was added when serializing
    let json_s: serde_json::Value = serde_json::from_str(&json_str_s).unwrap();
    assert_eq!(json_s[0], 33);
}
