// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::Value;
use std::ops;

/// A type that can be used to index into a `bin_prot::Value`.
pub trait Index {
    /// Return None if the key is not already in the array or object.
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
}

// Numeric indexing, only compatible with List and Tuple (or sum types containing either of these)
impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::List(ref vec) | Value::Tuple(ref vec) => vec.get(*self),
            Value::Sum { ref value, .. } => match **value {
                Value::List(ref vec) | Value::Tuple(ref vec) => vec.get(*self),
                _ => None,
            },
            _ => None,
        }
    }
}

// String indexing. Only compatible with Record (or sum types containg a record)
impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Record(ref map) => map.iter().find(|(k, _v)| k == self).map(|(_k, v)| v),
            Value::Sum { ref value, .. } => match **value {
                Value::Record(ref map) => map.iter().find(|(k, _v)| k == self).map(|(_k, v)| v),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
}

impl<I> ops::Index<I> for Value
where
    I: Index + std::fmt::Display,
{
    type Output = Value;

    // The ops::Index will panic when indexing a value that doesn't exist
    // This is consistent to the behaviour when indexing into an array
    fn index(&self, index: I) -> &Value {
        index
            .index_into(self)
            .unwrap_or_else(|| panic!("No value for index: {}", index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn index_into_list() {
        let val = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(val[0], Value::Int(1));
        assert_eq!(val[1], Value::Int(2));
        assert_eq!(val[2], Value::Int(3));
    }

    #[test]
    #[should_panic(expected = "No value for index: 3")]
    fn index_out_of_bounds_panics() {
        let val = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        let _ = val[3];
    }

    #[test]
    fn index_into_record() {
        let inner = vec![
            ("one".to_string(), Value::Int(1)),
            ("two".to_string(), Value::Int(2)),
        ];
        let val = Value::Record(inner);
        assert_eq!(val["one"], Value::Int(1));
        assert_eq!(val["two"], Value::Int(2));
    }

    #[test]
    #[should_panic(expected = "No value for index: missing")]
    fn no_value_for_key_panics() {
        let inner = vec![
            ("one".to_string(), Value::Int(1)),
            ("two".to_string(), Value::Int(2)),
        ];
        let val = Value::Record(inner);
        let _ = val["missing"];
    }

    #[test]
    fn index_into_tuple_variants() {
        let val = Value::Sum {
            name: "variant A".to_string(),
            index: 0,
            value: Box::new(Value::List(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
            ])),
        };
        assert_eq!(val[0], Value::Int(1));
        assert_eq!(val[1], Value::Int(2));
        assert_eq!(val[2], Value::Int(3));
    }

    #[test]
    fn index_into_record_variants() {
        let inner = vec![
            ("one".to_string(), Value::Int(1)),
            ("two".to_string(), Value::Int(2)),
        ];

        let val = Value::Sum {
            name: "variant A".to_string(),
            index: 0,
            value: Box::new(Value::Record(inner)),
        };
        assert_eq!(val["one"], Value::Int(1));
        assert_eq!(val["two"], Value::Int(2));
    }

    #[test]
    fn nested_indexing() {
        let inner = vec![("B".to_string(), Value::Int(1))];
        let val = Value::Record(inner);

        let mut outer = HashMap::new();
        outer.insert("A".to_string(), val);

        assert_eq!(outer["A"]["B"], Value::Int(1));
    }

    #[test]
    fn can_access_option_inner() {
        let val = Value::Option(None);
        assert!(val.inner().is_none());
    }

    #[test]
    #[should_panic(expected = "Called inner on a non-option variant Int(1)")]
    fn calling_inner_on_non_option_panics() {
        let val = Value::Int(1);
        assert!(val.inner().is_none());
    }
}
