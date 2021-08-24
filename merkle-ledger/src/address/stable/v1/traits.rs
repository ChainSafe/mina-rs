// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use direction::Direction;

pub trait Address {
    type T: PartialEq + Eq + std::hash::Hash;

    fn of_byte_string(&self, str: &str) -> Self::T;
    fn of_directions(&self, directions: &[Direction]) -> Self::T;
    fn root(&self) -> Self::T;
    fn slice(&self, start: usize, end: usize) -> Self::T;
    fn get(&self, pos: usize) -> Self::T;
    fn copy(other: &Self::T) -> Self::T;
    fn parent(&self) -> anyhow::Result<Self::T>;
    fn child(&self, ledger_depth: usize, direction: Direction) -> anyhow::Result<Self::T>;
    fn child_exn(&self, ledger_depth: usize, direction: Direction) -> Self::T;
    fn parent_exn(&self) -> Self::T;
    fn dirs_from_root(&self, other: Self::T) -> Vec<Direction>;
    fn sibling(&self, other: Self::T) -> Self::T;
    fn next(&self) -> Option<Self::T>;
    fn prev(&self) -> Option<Self::T>;
    fn is_leaf(&self, ledger_depth: usize, other: Self::T) -> bool;
    fn is_parent_of(&self, maybe_child: Self::T) -> bool;
    fn serialize(&self, ledger_depth: usize) -> Vec<u8>;
    fn to_string(&self) -> String;
    // fn pp(&self, format: Formater);
}

pub trait Range {
    type T0: PartialEq + Eq + std::hash::Hash;
    type T1: PartialEq + Eq + std::hash::Hash;
    // fn fold;
    // fn subtree_range(&self, ledger_depth: usize);
    // fn subtree_range_seq(&self, ledger_depth: usize) -> Self::T;
    // fn depth(&self) -> Self::T;
    // fn height(&self, ledger_depth:usize) -> usize;
    // fn to_int(&self) -> usize;
    // fn of_int_exn(&self, ledger_depth: usize, other: usize) -> Self::T;
}
