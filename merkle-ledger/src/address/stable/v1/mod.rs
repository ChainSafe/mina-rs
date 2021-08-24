// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bitvec::prelude::*;

use self::traits::Address;

pub mod traits;

pub struct MerkleAddress {}

impl Address for MerkleAddress {
    type T = BitVec;

    fn of_byte_string(&self, str: &str) -> Self::T {
        todo!()
    }

    fn of_directions(&self, directions: &[direction::Direction]) -> Self::T {
        todo!()
    }

    fn root(&self) -> Self::T {
        todo!()
    }

    fn slice(&self, start: usize, end: usize) -> Self::T {
        todo!()
    }

    fn get(&self, pos: usize) -> Self::T {
        todo!()
    }

    fn copy(other: &Self::T) -> Self::T {
        todo!()
    }

    fn parent(&self) -> anyhow::Result<Self::T> {
        todo!()
    }

    fn child(
        &self,
        ledger_depth: usize,
        direction: direction::Direction,
    ) -> anyhow::Result<Self::T> {
        todo!()
    }

    fn child_exn(&self, ledger_depth: usize, direction: direction::Direction) -> Self::T {
        todo!()
    }

    fn parent_exn(&self) -> Self::T {
        todo!()
    }

    fn dirs_from_root(&self, other: Self::T) -> Vec<direction::Direction> {
        todo!()
    }

    fn sibling(&self, other: Self::T) -> Self::T {
        todo!()
    }

    fn next(&self) -> Option<Self::T> {
        todo!()
    }

    fn prev(&self) -> Option<Self::T> {
        todo!()
    }

    fn is_leaf(&self, ledger_depth: usize, other: Self::T) -> bool {
        todo!()
    }

    fn is_parent_of(&self, maybe_child: Self::T) -> bool {
        todo!()
    }

    fn serialize(&self, ledger_depth: usize) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
