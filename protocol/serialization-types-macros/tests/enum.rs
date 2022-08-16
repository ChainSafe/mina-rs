// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_serialization_types_macros::*;
    use versioned::Versioned;

    #[derive(Debug, Clone, Eq, PartialEq, derive_more::From, derive_more::Into)]
    struct I64(pub i64);

    #[derive(Debug, Clone, Eq, PartialEq, AutoFrom)]
    #[auto_from(Bar)]
    enum Foo {
        V1,
        V2(i64),
        V3(i64, Option<i64>, Vec<i64>, Box<i64>),
        V4 {
            f1: i64,
        },
        V5 {
            f1: i64,
            f2: Option<i64>,
            f3: Vec<i64>,
            f4: Box<i64>,
        },
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    enum Bar {
        V1,
        V2(I64),
        V3(I64, Option<I64>, Vec<I64>, Box<I64>),
        V4 {
            f1: I64,
        },
        V5 {
            f1: I64,
            f2: Option<I64>,
            f3: Vec<I64>,
            f4: Box<I64>,
        },
    }

    type BarV1 = Versioned<Bar, 1>;

    #[test]
    fn enum_roundtrip() {
        enum_roundtrip_inner(Foo::V1);
        enum_roundtrip_inner(Foo::V2(1));
        enum_roundtrip_inner(Foo::V3(2, Some(3), vec![4, 5], Box::new(6)));
        enum_roundtrip_inner(Foo::V4 { f1: 7 });
        enum_roundtrip_inner(Foo::V5 {
            f1: 8,
            f2: Some(9),
            f3: vec![10, 11, 12],
            f4: Box::new(13),
        });
    }

    fn enum_roundtrip_inner(foo: Foo) {
        let bar: Bar = foo.clone().into();
        let bar_v1: BarV1 = foo.clone().into();
        let foo_from_bar: Foo = bar.into();
        let foo_from_bar_v1: Foo = bar_v1.into();
        assert_eq!(foo, foo_from_bar);
        assert_eq!(foo, foo_from_bar_v1);
    }
}
