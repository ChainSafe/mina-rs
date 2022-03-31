// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::prefixes::*;
use ark_ff::BigInteger256;
use lockfree_object_pool::{SpinLockObjectPool, SpinLockReusable};
use mina_hasher::{create_legacy, Fp, Hashable, Hasher, PoseidonHasherLegacy, ROInput};
use num::{BigUint, Num};
use once_cell::sync::OnceCell;

/// Trait that merges the hashes of child nodes
/// and calculates the hash of their parent
/// degree defaults to 2
pub trait MerkleMerger<const DEGREE: usize = DEFAULT_DEGREE> {
    /// Type that represents the hash value
    type Hash;
    /// Merges hashes of child nodes,
    /// with metadata of the target node
    fn merge(
        hashes: [Option<Self::Hash>; DEGREE],
        metadata: MerkleTreeNodeMetadata<DEGREE>,
    ) -> Option<Self::Hash>;
}

/// Merger for mina binary merkle tree that uses poseidon hash
/// with mina specific domain string calculated from node height
pub struct MinaPoseidonMerkleMerger {}

impl MerkleMerger for MinaPoseidonMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; MINA_POSEIDON_MERKLE_DEGREE],
        metadata: MerkleTreeNodeMetadata<MINA_POSEIDON_MERKLE_DEGREE>,
    ) -> Option<Self::Hash> {
        merge_poseidon_hash(hashes, metadata.height()).into()
    }
}

#[derive(Clone)]
struct MinaPoseidonMerkleTreeNonLeafNode([Option<Fp>; MINA_POSEIDON_MERKLE_DEGREE], u32);

impl Hashable for MinaPoseidonMerkleTreeNonLeafNode {
    type D = u32;

    fn to_roinput(&self) -> mina_hasher::ROInput {
        let mut roi = ROInput::new();
        for hash in self.0.into_iter().flatten() {
            roi.append_field(hash);
        }
        roi
    }

    fn domain_string(height: Self::D) -> Option<String> {
        // use height - 1 here because in mina leaf nodes are not counted
        if height > 0 {
            Some(make_prefix_merkle_tree(height - 1))
        } else {
            None
        }
    }
}

fn merge_poseidon_hash(hashes: [Option<Fp>; MINA_POSEIDON_MERKLE_DEGREE], height: u32) -> Fp {
    static HASHER_POOL: OnceCell<
        SpinLockObjectPool<PoseidonHasherLegacy<MinaPoseidonMerkleTreeNonLeafNode>>,
    > = OnceCell::new();
    // Not calling reset here because `hasher.init` is called after `pull`, which implicitly calls sponge.reset()
    let pool = HASHER_POOL.get_or_init(|| SpinLockObjectPool::new(|| create_legacy(0), |_| ()));
    let mut hasher = pool.pull();
    merge_poseidon_hash_with_hasher(&mut hasher, hashes, height)
}

fn merge_poseidon_hash_with_hasher(
    hasher: &mut SpinLockReusable<PoseidonHasherLegacy<MinaPoseidonMerkleTreeNonLeafNode>>,
    hashes: [Option<Fp>; MINA_POSEIDON_MERKLE_DEGREE],
    height: u32,
) -> Fp {
    let mut flatten_hashes = hashes;
    for hash_opt in flatten_hashes.iter_mut() {
        if hash_opt.is_none() {
            *hash_opt = get_empty_hash(hasher, height - 1).into();
        }
    }
    let hashable = MinaPoseidonMerkleTreeNonLeafNode(flatten_hashes, height);
    hasher.init(height);
    hasher.hash(&hashable)
}

fn get_empty_hash(
    hasher: &mut SpinLockReusable<PoseidonHasherLegacy<MinaPoseidonMerkleTreeNonLeafNode>>,
    height: u32,
) -> Fp {
    if height == 0 {
        static EMPTY_HASH: OnceCell<Fp> = OnceCell::new();
        *EMPTY_HASH.get_or_init(|| {
            let big = BigUint::from_str_radix(
                /*
                    This is From OCaml code,
                    add below code to genesis_ledger_helper.ml and run dune test

                    let%test_unit "empty hash" =
                        let empty =
                            Snark_params.Tick.Field.to_string Mina_base.Account.empty_digest
                        in
                        print_string empty
                */
                "14604874247461951431777712543359658136906556694369689076707549712589474483312",
                10,
            )
            .expect("Failed to parse BigUint");
            let big256: BigInteger256 = big
                .try_into()
                .expect("Failed to convert BigUint to BigInteger256");
            big256.into()
        })
    } else {
        let child_hash = get_empty_hash(hasher, height - 1);
        merge_poseidon_hash_with_hasher(hasher, [Some(child_hash), Some(child_hash)], height)
    }
}
