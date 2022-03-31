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
pub struct MinaPoseidonMerkleHasher<TItem>
where
    TItem: mina_hasher::Hashable,
    <TItem as mina_hasher::Hashable>::D: Default,
{
    _pd: PhantomData<TItem>,
}

impl<TItem> MerkleHasher<MINA_POSEIDON_MERKLE_DEGREE> for MinaPoseidonMerkleHasher<TItem>
where
    TItem: mina_hasher::Hashable,
    <TItem as mina_hasher::Hashable>::D: Default,
{
    type Item = TItem;
    type Hash = Fp;
    fn hash(
        item: &Self::Item,
        _: MerkleTreeNodeMetadata<MINA_POSEIDON_MERKLE_DEGREE>,
    ) -> Self::Hash {
        use mina_hasher::Hasher;
        // TODO: get hasher pool with `type_id = `std::any::type_name::<Self::Item>()`;
        let mut hasher = mina_hasher::create_legacy(Default::default());
        hasher.hash(item)
    }
}
