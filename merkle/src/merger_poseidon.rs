// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::prefixes::*;
use ark_ff::BigInteger256;
use mina_hasher::{create_legacy, Fp, Hashable, Hasher, ROInput};
use once_cell::sync::OnceCell;
use proof_systems::mina_hasher::create_kimchi;

/// Merger for mina binary merkle tree that uses poseidon hash
/// with mina specific domain string calculated from node height
pub struct MinaPoseidonMerkleMerger;

impl MerkleMerger for MinaPoseidonMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; 2],
        metadata: MerkleTreeNodeMetadata,
    ) -> Option<Self::Hash> {
        merge_poseidon_hash_legacy(hashes, metadata.height()).into()
    }
}

/// Merger for mina binary merkle tree that uses poseidon hash
/// with mina specific domain string calculated from node height
pub struct MinaPoseidonKimchiMerkleMerger;

impl MerkleMerger for MinaPoseidonKimchiMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; 2],
        metadata: MerkleTreeNodeMetadata,
    ) -> Option<Self::Hash> {
        merge_poseidon_hash_kimchi(hashes, metadata.height()).into()
    }
}

#[derive(Clone)]
struct MinaPoseidonMerkleTreeNonLeafNode([Option<Fp>; 2], u32);

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

fn merge_poseidon_hash_legacy(mut hashes: [Option<Fp>; 2], height: u32) -> Fp {
    for hash_opt in hashes.iter_mut() {
        if hash_opt.is_none() {
            *hash_opt = get_empty_hash_legacy(height - 1).into();
        }
    }
    let mut hasher = create_legacy(height);
    let hashable = MinaPoseidonMerkleTreeNonLeafNode(hashes, height);
    hasher.hash(&hashable)
}

fn get_empty_hash_legacy(height: u32) -> Fp {
    if height == 0 {
        static EMPTY_HASH: OnceCell<Fp> = OnceCell::new();
        *EMPTY_HASH.get_or_init(|| {
            /*
                This is From OCaml code,
                add below code to genesis_ledger_helper.ml and run dune test

                let%test_unit "empty hash" =
                    let empty =
                        Snark_params.Tick.Field.to_string Mina_base.Account.empty_digest
                    in
                    print_string empty

                radix 10: 14604874247461951431777712543359658136906556694369689076707549712589474483312
                hex:      0x204a10dde313dedb9a8a568d92ad6df0eecaff98ed379ae50896824fa1dbcc70
            */
            const DEFAULT_HASH: BigInteger256 = BigInteger256::new([
                618825277339585648,
                17206846358602357477,
                11135808194678189552,
                2326690702673829595,
            ]);
            DEFAULT_HASH.into()
        })
    } else {
        let child_hash = get_empty_hash_legacy(height - 1);
        merge_poseidon_hash_legacy([Some(child_hash), Some(child_hash)], height)
    }
}

fn merge_poseidon_hash_kimchi(mut hashes: [Option<Fp>; 2], height: u32) -> Fp {
    for hash_opt in hashes.iter_mut() {
        if hash_opt.is_none() {
            *hash_opt = get_empty_hash_kimchi(height - 1).into();
        }
    }
    let mut hasher = create_kimchi(height);
    let hashable = MinaPoseidonMerkleTreeNonLeafNode(hashes, height);
    hasher.hash(&hashable)
}

fn get_empty_hash_kimchi(height: u32) -> Fp {
    if height == 0 {
        static EMPTY_HASH: OnceCell<Fp> = OnceCell::new();
        *EMPTY_HASH.get_or_init(|| {
            /*
                This is From OCaml code,
                add below code to genesis_ledger_helper.ml and run dune test

                let%test_unit "empty hash" =
                    let empty =
                        Snark_params.Tick.Field.to_string Mina_base.Account.empty_digest
                    in
                    print_string empty

                radix 10: 12389486750955908784174944900536707123964862344774386303053831163567322538302
                hex:      0x1b6433749ae531d94f84d305a3441035072f4835c802553abc5f8be4b595a13e
            */
            const DEFAULT_HASH: BigInteger256 = BigInteger256::new([
                13573721616333250878,
                517711877996959034,
                5729936647089557557,
                1973759112603054553,
            ]);
            DEFAULT_HASH.into()
        })
    } else {
        let child_hash = get_empty_hash_kimchi(height - 1);
        merge_poseidon_hash_kimchi([Some(child_hash), Some(child_hash)], height)
    }
}
