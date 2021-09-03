// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bitvec::prelude::*;

use self::traits::Address;

pub mod traits;

pub struct MerkleAddress {}

impl Address for MerkleAddress {
    type T = BitVec;

    fn of_byte_string(&self, _str: &str) -> Self::T {
        todo!()
    }

    fn of_directions(&self, _directions: &[direction::Direction]) -> Self::T {
        todo!()
    }

    fn root(&self) -> Self::T {
        todo!()
    }

    fn slice(&self, _start: usize, _end: usize) -> Self::T {
        todo!()
    }

    fn get(&self, _pos: usize) -> Self::T {
        todo!()
    }

    fn copy(_other: &Self::T) -> Self::T {
        todo!()
    }

    fn parent(&self) -> anyhow::Result<Self::T> {
        todo!()
    }

    fn child(
        &self,
        _ledger_depth: usize,
        _direction: direction::Direction,
    ) -> anyhow::Result<Self::T> {
        todo!()
    }

    fn child_exn(&self, _ledger_depth: usize, _direction: direction::Direction) -> Self::T {
        todo!()
    }

    fn parent_exn(&self) -> Self::T {
        todo!()
    }

    fn dirs_from_root(&self, _other: Self::T) -> Vec<direction::Direction> {
        todo!()
    }

    fn sibling(&self, _other: Self::T) -> Self::T {
        todo!()
    }

    fn next(&self) -> Option<Self::T> {
        todo!()
    }

    fn prev(&self) -> Option<Self::T> {
        todo!()
    }

    fn is_leaf(&self, _ledger_depth: usize, _other: Self::T) -> bool {
        todo!()
    }

    fn is_parent_of(&self, _maybe_child: Self::T) -> bool {
        todo!()
    }

    fn serialize(&self, _ledger_depth: usize) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn depth(_this: &Self::T) -> usize {
        todo!()
    }

    fn height(_ledger_depth: usize, _this: &Self::T) -> usize {
        todo!()
    }
}
