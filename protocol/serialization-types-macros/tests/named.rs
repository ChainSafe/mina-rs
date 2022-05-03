// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_serialization_types_macros::*;

    #[derive(Debug, Clone, PartialEq, derive_more::From, derive_more::Into)]
    struct I64(pub i64);

    #[derive(Debug, Clone, PartialEq, AutoFrom)]
    #[auto_from(Bar)]
    #[auto_from(Bar2)]
    struct Foo {
        pub f1: i64,
        pub f2: i64,
        f3: i64,
        f4: Vec<i64>,
        f5: Option<i64>,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Bar {
        pub f1: I64,
        f2: I64,
        pub f3: I64,
        f4: Vec<I64>,
        f5: Option<I64>,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Bar2 {
        f1: I64,
        f2: I64,
        f3: I64,
        f4: Vec<I64>,
        f5: Option<I64>,
    }

    type BarV1 = ::versioned::Versioned<Bar, 1>;
    type Bar2V1 = ::versioned::Versioned<BarV1, 1>;

    #[test]
    fn struct_with_named_fields_roundtrip() {
        let foo = Foo {
            f1: 3,
            f2: 4,
            f3: 5,
            f4: vec![7, 8, 9],
            f5: Some(10),
        };

        let bar: Bar = foo.clone().into();
        let bar2: Bar2 = foo.clone().into();
        let foo_from_bar: Foo = bar.into();
        let foo_from_bar2: Foo = bar2.into();

        assert_eq!(foo, foo_from_bar);
        assert_eq!(foo, foo_from_bar2);
    }

    #[test]
    fn struct_with_named_fields_roundtrip_versioned() {
        let foo = Foo {
            f1: 3,
            f2: 4,
            f3: 5,
            f4: vec![7, 8, 9],
            f5: Some(10),
        };

        let bar: BarV1 = foo.clone().into();
        let bar2: Bar2V1 = foo.clone().into();
        let foo_from_bar: Foo = bar.into();
        let foo_from_bar2: Foo = bar2.into();

        assert_eq!(foo, foo_from_bar);
        assert_eq!(foo, foo_from_bar2);
    }
}
