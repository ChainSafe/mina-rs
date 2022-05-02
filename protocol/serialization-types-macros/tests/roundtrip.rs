// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_serialization_types_macros::*;

    #[derive(Debug, Clone, PartialEq, derive_more::From, derive_more::Into)]
    struct I64(pub i64);

    #[derive(Debug, Clone, PartialEq)]
    struct Foo {
        pub f1: i64,
        pub f2: i64,
        f3: i64,
        f4: Vec<i64>,
        f5: Option<i64>,
    }

    #[derive(Debug, Clone, PartialEq, AutoFrom)]
    #[auto_from(Foo)]
    struct Bar {
        pub f1: I64,
        f2: I64,
        pub f3: I64,
        f4: Vec<I64>,
        f5: Option<I64>,
    }

    #[test]
    fn roundtrip() {
        let foo = Foo {
            f1: 3,
            f2: 4,
            f3: 5,
            f4: vec![7, 8, 9],
            f5: Some(10),
        };

        let bar: Bar = foo.clone().into();
        let foo2: Foo = bar.into();

        assert_eq!(foo, foo2);
    }
}
