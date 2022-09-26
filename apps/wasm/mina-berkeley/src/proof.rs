// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_merkle::DefaultMerkleProof as MinaDefaultMerkleProof;
use mina_merkle::*;
use mina_network::processor::{DummyAccount, DummyHasher};
use proof_systems::mina_hasher::Fp;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DefaultMerkleProof {
    _index: usize,
    _item: DummyAccount,
    _peer_indices: Vec<usize>,
    _peer_hashes: Vec<Option<Fp>>,
    _hasher: PhantomData<DummyHasher>,
    _merger: PhantomData<MinaPoseidonMerkleMerger>,
}

impl From<&MinaDefaultMerkleProof<DummyAccount, Fp, DummyHasher, MinaPoseidonMerkleMerger>>
    for DefaultMerkleProof
{
    fn from(
        v: &MinaDefaultMerkleProof<DummyAccount, Fp, DummyHasher, MinaPoseidonMerkleMerger>,
    ) -> Self {
        DefaultMerkleProof {
            _index: v.index,
            _item: v.item.clone(),
            _peer_indices: v.peer_indices.clone(),
            _peer_hashes: v.peer_hashes.clone(),
            _hasher: Default::default(),
            _merger: Default::default(),
        }
    }
}
