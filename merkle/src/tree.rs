// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait MerkleTree {
    type Item;
    type Hash;

    fn depth(&self) -> u32;
    fn count(&self) -> usize;
    fn root(&mut self) -> Option<Self::Hash>;

    fn add(&mut self, item: Self::Item);
    fn add_batch(&mut self, items: Vec<Self::Item>);
}
