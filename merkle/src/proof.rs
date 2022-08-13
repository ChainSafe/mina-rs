// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! This module contains traits, structs and utilities of merkle proof

use std::{error::Error, marker::PhantomData};

use crate::*;

/// Merkle proof trait of a single leaf node, for details refer to <https://www.webopedia.com/definitions/merkle-proof/>
pub trait MerkleProof {
    /// Hash type
    type Hash: PartialEq;

    /// Error type
    type Error: Error;

    /// Calculates the root hash
    fn root_hash(&self) -> Result<Self::Hash, Self::Error>;

    /// Verifies if the proof is valid
    fn verify(&self, root_hash: &Self::Hash) -> bool {
        if let Ok(hash) = self.root_hash() {
            &hash == root_hash
        } else {
            false
        }
    }
}

/// Type that represents errors in calculating hashes for a merkle proof
#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum MerkleProofError {
    /// Index of a data node should be positive
    #[error("Index of a data node should be positive")]
    InvalidIndex,
    /// The merkle proof is invalid
    #[error("The merkle proof is invalid")]
    InvalidProof,
    /// Errors occur in hash merger
    #[error("Errors occur in hash merger")]
    MergerFailure,
}

/// Merkle proof implementation of a single leaf node, for details refer to <https://www.webopedia.com/definitions/merkle-proof/>
pub struct DefaultMerkleProof<Item, Hash, Hasher, Merger>
where
    Hash: PartialEq + Clone,
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
{
    index: usize,
    item: Item,
    peer_indices: Vec<usize>,
    peer_hashes: Vec<Option<Hash>>,
    _hasher: PhantomData<Hasher>,
    _merger: PhantomData<Merger>,
}

impl<Item, Hash, Hasher, Merger> DefaultMerkleProof<Item, Hash, Hasher, Merger>
where
    Hash: PartialEq + Clone,
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
{
    /// Creates merkle proof instance
    /// index is the node index counted from root. e.g. index of the root node is 0
    /// Note that index of a data node will always be positive
    pub fn new(
        index: usize,
        item: Item,
        peer_indices: Vec<usize>,
        peer_hashes: Vec<Option<Hash>>,
    ) -> Self {
        assert!(index > 0, "index of a data node should always to positive");
        assert_eq!(
            peer_indices.len(),
            peer_hashes.len(),
            "length of peer_indices and peer_hashes should match"
        );
        assert!(
            !peer_indices.is_empty(),
            "length of peer_indices and peer_hashes should be positive"
        );
        Self {
            index,
            item,
            peer_indices,
            peer_hashes,
            _hasher: Default::default(),
            _merger: Default::default(),
        }
    }
}

impl<Item, Hash, Hasher, Merger> MerkleProof for DefaultMerkleProof<Item, Hash, Hasher, Merger>
where
    Hash: PartialEq + Clone + std::fmt::Debug,
    Hasher: MerkleHasher<Item = Item, Hash = Hash>,
    Merger: MerkleMerger<Hash = Hash>,
{
    type Hash = Hash;
    type Error = MerkleProofError;

    fn root_hash(&self) -> Result<Hash, Self::Error> {
        // index of a data node should always to positive
        if self.index > 0 {
            // 1. Get the index and hash of the data node
            let mut index = self.index;
            let mut hash_opt = Some(Hasher::hash(
                &self.item,
                MerkleTreeNodeMetadata::new(self.index, self.peer_indices.len() as u32),
            ));
            for i in 0..self.peer_indices.len() {
                // 2. Find the hash of its sibling
                let peer_index = self.peer_indices[i];
                let peer_hash_opt = &self.peer_hashes[i];
                // 3 Prepare the input for for the hash merger by ordering the 2 hashes properly
                let hashes =
                    get_ordered_siblings(index, hash_opt, peer_index, peer_hash_opt.clone())?;
                // 4 Calculates the hash of their parent by invoking the associated merkle merger
                let parent_index = get_parent_index(index);
                hash_opt = Merger::merge(
                    hashes,
                    MerkleTreeNodeMetadata::new(parent_index, self.peer_indices.len() as u32),
                );
                // Go back to step 1 and apply the same flow to this parent node
                // until the root hash (of index 0) has been calculated.
                index = parent_index;
            }
            if let Some(hash) = hash_opt {
                Ok(hash)
            } else {
                Err(MerkleProofError::MergerFailure)
            }
        } else {
            Err(MerkleProofError::InvalidIndex)
        }
    }
}

fn get_parent_index(index: usize) -> usize {
    debug_assert!(index > 0);
    (index - 1) / 2
}

fn get_ordered_siblings<Hash>(
    index: usize,
    hash: Option<Hash>,
    sibling_index: usize,
    sibling_hash: Option<Hash>,
) -> Result<[Option<Hash>; 2], MerkleProofError> {
    if index + 1 == sibling_index {
        Ok([hash, sibling_hash])
    } else if index == sibling_index + 1 {
        Ok([sibling_hash, hash])
    } else {
        Err(MerkleProofError::InvalidProof)
    }
}
