// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use mina_hasher::Fp;
use std::marker::PhantomData;

/// Trait for implementing merkle tree hasher
/// degree defaults to 2
pub trait MerkleHasher<const DEGREE: usize = DEFAULT_DEGREE> {
    /// Type that [MerkleHasher] calculates hash from
    type Item;
    /// Type that represents the hash value
    type Hash;
    /// Calculates hash from an item and its associated metadata
    fn hash(item: &Self::Item, metadata: MerkleTreeNodeMetadata<DEGREE>) -> Self::Hash;
}

/// Hasher for mina binary merkle tree that uses poseidon hash
pub struct MinaPoseidonMerkleHasher<TItem, const DEGREE: usize = DEFAULT_DEGREE>
where
    TItem: mina_hasher::Hashable,
    <TItem as mina_hasher::Hashable>::D: Default,
{
    _pd: PhantomData<TItem>,
}

impl<TItem, const DEGREE: usize> MerkleHasher<DEGREE> for MinaPoseidonMerkleHasher<TItem, DEGREE>
where
    TItem: mina_hasher::Hashable,
    <TItem as mina_hasher::Hashable>::D: Default,
{
    type Item = TItem;
    type Hash = Fp;
    fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata<DEGREE>) -> Self::Hash {
        use mina_hasher::Hasher;
        // FIXME: Get hasher from object pool
        // when https://github.com/o1-labs/proof-systems/pull/462/files is merged
        let mut hasher = mina_hasher::create_legacy(Default::default());
        hasher.hash(item)
    }
}
