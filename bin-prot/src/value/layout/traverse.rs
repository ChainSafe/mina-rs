// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This module implements the logic for traversing a layout in a way that corresponds to how the binary data is arranged out.
//! Since BinProtRule is a recursive data type the correct traversal is a depth first traversal of the tree defined by the data type
//! with a few important differences.
//!
//! The layout tree includes the concept of Sum(ocaml)/Enum(rust) types. These are nodes in the tree where only one branch should be taken
//! depending on which variant of the enum we are deserializing. When reading an enum from binary the first byte specifies which variant to deserialize.
//! Therefore out DFS iterator requires a way to accept input and branch at particular nodes in the tree.
//!
//! This is where the BranchingIterator trait comes in. A branching iterator is similar to a regular iterator but at some points in its iteration it requires
//! the calling code call a `branch` function to tell it which path to take.
//!
//! The other difference relates to deserializing variable length vector types. When deserializing a vector the length can be read from the binary and then
//! the `repeat(n)` method called on the iterator spefifying the length. It will then repeat the current node `n` times to allow it to be deserialized.
//!
//! The traversal of a layout should be done in parallel with reading a bin_prot encoded binary such that the type tree informs the deserializer how to read the
//! data and the data informs the traversal how it should handle enum types.
//! Combined this allows parsing of types defined by the layout into loosely typed representations.
//!

use crate::value::layout::{BinProtRule, Polyvar, RuleRef};

/// Implements a depth first search of the type tree
/// defined by a BinProtRule
pub struct BinProtRuleIterator {
    stack: Vec<BinProtRule>, // regular stack to implement the DFS
    // Tree nodes can branch (only one child should be followed) rather than require traversal of all children
    // If that is the case the parent should add the children to the branch field and the next path will be taken from here rather than the stack
    branch: Option<Vec<Vec<BinProtRule>>>,
    current_module_path: Option<String>, // holds on to most recent path encountered in traverse
}

/// An iterator where the next item may require specifying a branch to take
/// At some points in its iteration the iterator will require a call to branch to select which
/// path to take before continuing.
/// Also supports repeating a node in the tree for a given number of reps
pub trait BranchingIterator {
    type Item;
    type Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;
    fn branch(&mut self, branch: usize) -> Result<(), Self::Error>;
}

impl BranchingIterator for BinProtRuleIterator {
    type Item = BinProtRule;
    type Error = String;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.branch.is_some() {
            return Err("Must call branch to proceed".to_string());
        }

        let top = self.stack.pop();
        let r = top.clone();
        match top {
            Some(rule) => {
                match rule {
                    BinProtRule::Option(r) | BinProtRule::List(r) => {
                        // the code driving the iterator should call `repeat` if it encounters a list
                        self.stack.push(*r);
                    }
                    BinProtRule::Record(mut rules) => {
                        self.stack
                            .extend(rules.drain(0..).map(|field| field.field_rule).rev());
                    }
                    BinProtRule::Tuple(mut rules) => {
                        self.stack.extend(rules.drain(0..).rev());
                    }
                    BinProtRule::Sum(summands) => {
                        // don't add to the stack. Add to the branch field instead
                        // this must be resolved by calling `branch` before the iterator can continue
                        self.branch = Some(summands.into_iter().map(|s| s.ctor_args).collect());
                    }
                    BinProtRule::Polyvar(polyvars) => {
                        // these are pretty much anonymous enum/sum types and should be handled the same way
                        self.branch = Some(
                            polyvars
                                .into_iter()
                                .map(|s| match s {
                                    Polyvar::Tagged(pv) => pv.polyvar_args,
                                    Polyvar::Inherited(rule) => {
                                        vec![rule]
                                    }
                                })
                                .collect(),
                        );
                    }
                    BinProtRule::Reference(rule_ref) => match rule_ref {
                        RuleRef::Unresolved(_payload) => {
                            unimplemented!();
                        }
                        RuleRef::Resolved(payload) => {
                            self.stack.push(*payload.ref_rule);
                            self.current_module_path = Some(payload.source_module_path);
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
                            return Ok(Some(BinProtRule::CustomForPath(path.to_string(), rules)));
                        }
                    }
                    r => panic!("unimplemented: {:?}", r),
                };
                Ok(r)
            }
            None => Ok(None), // end of traversal
        }
    }

    fn branch(&mut self, branch: usize) -> Result<(), Self::Error> {
        if let Some(summands) = &self.branch {
            if branch >= summands.len() {
                return Err(format!(
                    "Invalid branch index. Given {}, Branch must be < {}",
                    branch,
                    summands.len()
                ));
            }
        }

        if let Some(mut branches) = self.branch.take() {
            let s = branches
                .get_mut(branch)
                .ok_or_else(|| "Invalid branch".to_string())?;
            self.stack.extend(s.drain(0..));
            Ok(())
        } else {
            Err("Cannot branch at this location in the tree".to_string())
        }
    }
}

impl BinProtRuleIterator {
    // takes whatever is next on the stack and repeats it to it appears `reps` times
    pub fn repeat(&mut self, reps: usize) {
        if let Some(top) = self.stack.pop() {
            self.stack.extend(std::iter::repeat(top).take(reps));
        }
    }

    // Drop a custom rule onto the stack
    pub fn push(&mut self, rule: BinProtRule) {
        self.stack.push(rule);
    }
}

// Consumes the rule and produces a branching iterator
impl BinProtRule {
    #[allow(dead_code)] // allow this for now since
    pub fn into_branching_iter(self) -> BinProtRuleIterator {
        BinProtRuleIterator {
            stack: vec![self],
            branch: None,
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
        let mut iter = layout.bin_prot_rule.into_branching_iter();
        while let Ok(Some(v)) = iter.next() {
            println!("{:?}\n", v);
        }
    }

    const TEST_LAYOUT_SUM: &str = include_str!("./test_layouts/sum_type_layout.json");

    #[test]
    fn test_layout_sum() {
        let layout: Layout = serde_json::from_str(TEST_LAYOUT_SUM).unwrap();
        let mut iter = layout.bin_prot_rule.into_branching_iter();
        // Test by taking the 0th branch at each branch node. Test is considered as pass
        // if no error
        loop {
            match iter.next() {
                Ok(Some(v)) => {
                    if let BinProtRule::Sum(_) = v {
                        // if its a sum type take the first variant in each case
                        iter.branch(0).expect("Invalid branch index");
                    }
                    println!("{:?}\n", v);
                }
                Err(e) => {
                    panic!("{}", e);
                }
                Ok(None) => {
                    println!("END!!!!");
                    break;
                }
            }
        }
    }
}
