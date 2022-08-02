// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use mina_hasher::{Fp, Hashable};
use proof_systems::mina_hasher::{create_kimchi, create_legacy, Hasher};
use std::marker::PhantomData;

/// Hasher for mina binary merkle tree that uses poseidon hash
pub struct MinaPoseidonMerkleHasherLegacy<Item>
where
    Item: mina_hasher::Hashable,
{
    _pd: PhantomData<Item>,
}

impl<Item> MerkleHasher for MinaPoseidonMerkleHasherLegacy<Item>
where
    Item: mina_hasher::Hashable,
    <Item as mina_hasher::Hashable>::D: Default,
{
    type Item = Item;
    type Hash = Fp;
    fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata) -> Self::Hash {
        let mut hasher = create_legacy(<Item as Hashable>::D::default());
        hasher.hash(item)
    }
}

/// Hasher for mina binary merkle tree that uses kimchi poseidon hash
pub struct MinaPoseidonMerkleHasher<Item>
where
    Item: mina_hasher::Hashable,
{
    _pd: PhantomData<Item>,
}

impl<Item> MerkleHasher for MinaPoseidonMerkleHasher<Item>
where
    Item: mina_hasher::Hashable,
    <Item as mina_hasher::Hashable>::D: Default,
{
    type Item = Item;
    type Hash = Fp;
    fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata) -> Self::Hash {
        let mut hasher = create_kimchi(<Item as Hashable>::D::default());
        hasher.hash(item)
    }
}
