// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This module implements the logic for traversing a layout in a way that corresponds to how the binary data is arranged out.
//! Since BinProtRule is a recursive data type the correct traversal is a depth first traversal of the tree defined by the data type
//! with a few important differences.
//!
//! The layout tree includes the concept of Sum(ocaml)/Enum(rust) types. These are nodes in the tree where only one branch should be taken
//! depending on which variant of the enum we are deserializing. When reading an enum from binary the first byte specifies which variant to deserialize.
//! It is the responsibility of the driving code to handle when Sum/Option rules are encountered and push the correct next rule to the stack.
//!
//! The other interesting case is deserializing variable length vector types. When deserializing a vector the length can be read from the binary and then
//! the element rule pushed back to the stack using the `push_n` method. It will then repeat the given node `n` times to allow it to be deserialized.
//!
//! The traversal of a layout should be done in parallel with reading a bin_prot encoded binary such that the type tree informs the deserializer how to read the
//! data and the data informs the traversal how it should handle enum types.
//! Combined this allows parsing of types defined by the layout into loosely typed representations.
//!

use crate::value::layout::{BinProtRule, RuleRef};

/// Implements a depth first search of the type tree
/// defined by a BinProtRule
pub struct BinProtRuleIterator {
    pub(crate) stack: Vec<BinProtRule>, // regular stack to implement the DFS
    current_module_path: Option<String>, // holds on to most recent path encountered in traverse
}

impl Iterator for BinProtRuleIterator {
    type Item = BinProtRule;

    fn next(&mut self) -> Option<Self::Item> {
        let top = self.stack.pop();
        let r = top.clone();
        match top {
            Some(rule) => {
                match rule {
                    BinProtRule::List(_rule) => {
                        // the code driving the iterator should take the list rule, push it
                        // then call repeat the required number of times
                    }
                    BinProtRule::Record(mut rules) => {
                        self.stack
                            .extend(rules.drain(0..).map(|field| field.field_rule).rev());
                    }
                    BinProtRule::Tuple(mut rules) => {
                        self.stack.extend(rules.drain(0..).rev());
                    }
                    BinProtRule::Sum(_) | BinProtRule::Polyvar(_) => {
                        // don't add to the stack. Add to the branch field instead
                        // this must be resolved by calling `branch` before the iterator can continue
                    }
                    BinProtRule::Option(_r) => {
                        // Option is a special case of a Sum where the None variant contain nothing
                    }
                    BinProtRule::Reference(rule_ref) => match rule_ref {
                        RuleRef::Unresolved(_payload) => {
                            unimplemented!();
                        }
                        RuleRef::Resolved(payload) => {
                            self.stack.push(*payload.ref_rule);
                            self.current_module_path = Some(payload.source_module_path);
                            return self.next();
                        }
                    },
                    BinProtRule::String
                    | BinProtRule::Int
                    | BinProtRule::Int64
                    | BinProtRule::Nat0
                    | BinProtRule::Bool
                    | BinProtRule::Unit
                    | BinProtRule::Char
                    | BinProtRule::Int32
                    | BinProtRule::NativeInt
                    | BinProtRule::Float => {} // These are leaves so nothing required
                    BinProtRule::Custom(rules) => {
                        if let Some(path) = &self.current_module_path {
                            return Some(BinProtRule::CustomForPath(path.to_string(), rules));
                        }
                    }
                    _ => unimplemented!(),
                };
                r
            }
            None => None, // end of traversal
        }
    }
}

impl BinProtRuleIterator {
    // Drop a custom rule onto the stack
    pub fn push(&mut self, rules: Vec<BinProtRule>) {
        self.stack.extend(rules);
    }

    // Drop a custom rule onto the stack n times
    pub fn push_n(&mut self, rule: BinProtRule, n: usize) {
        self.stack.extend(std::iter::repeat(rule).take(n));
    }
}

impl IntoIterator for BinProtRule {
    type Item = BinProtRule;
    type IntoIter = BinProtRuleIterator;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        BinProtRuleIterator {
            stack: vec![self],
            current_module_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::layout::Layout;
    const TEST_LAYOUT: &str = include_str!("./test_layouts/mina_state_hash_tuple.json");

    #[test]
    fn test_layout() {
        let layout: Layout = serde_json::from_str(TEST_LAYOUT).unwrap();
        let mut iter = layout.bin_prot_rule.into_iter();
        while let Some(v) = iter.next() {
            println!("{:?}\n", v);
        }
    }

    const TEST_LAYOUT_SUM: &str = include_str!("./test_layouts/sum_type_layout.json");

    #[test]
    fn test_layout_sum() {
        let layout: Layout = serde_json::from_str(TEST_LAYOUT_SUM).unwrap();
        let mut iter = layout.bin_prot_rule.into_iter();
        // Test by taking the 0th branch at each branch node. Test is considered as pass
        // if no error
        loop {
            match iter.next() {
                Some(v) => {
                    if let BinProtRule::Sum(summands) = v {
                        // if its a sum type take the first variant in each case
                        iter.push(summands[0].ctor_args.clone())
                    }
                }
                None => {
                    println!("END!!!!");
                    break;
                }
            }
        }
    }
}
