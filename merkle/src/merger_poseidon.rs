// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::prefixes::*;
use ark_ff::{BigInteger256, FromBytes};
use lockfree_object_pool::{SpinLockObjectPool, SpinLockReusable};
use mina_hasher::{create_legacy, Fp, Hashable, Hasher, PoseidonHasherLegacy, ROInput};
use once_cell::sync::OnceCell;

/// Merger for mina binary merkle tree that uses poseidon hash
/// with mina specific domain string calculated from node height
pub struct MinaPoseidonMerkleMerger;

impl MerkleMerger for MinaPoseidonMerkleMerger {
    type Hash = Fp;
    fn merge(
        hashes: [Option<Self::Hash>; 2],
        metadata: MerkleTreeNodeMetadata,
    ) -> Option<Self::Hash> {
        merge_poseidon_hash(hashes, metadata.height()).into()
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

fn merge_poseidon_hash(hashes: [Option<Fp>; 2], height: u32) -> Fp {
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
    mut hashes: [Option<Fp>; 2],
    height: u32,
) -> Fp {
    for hash_opt in hashes.iter_mut() {
        if hash_opt.is_none() {
            *hash_opt = get_empty_hash(hasher, height - 1).into();
        }
    }
    let hashable = MinaPoseidonMerkleTreeNonLeafNode(hashes, height);
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
            const BYTES_LE: [u8; 32] = [
                112, 204, 219, 161, 79, 130, 150, 8, 229, 154, 55, 237, 152, 255, 202, 238, 240,
                109, 173, 146, 141, 86, 138, 154, 219, 222, 19, 227, 221, 16, 74, 32,
            ];
            BigInteger256::read(BYTES_LE.as_slice())
                .expect("Failed to convert bytes to BigInteger256")
                .into()
        })
    } else {
        let child_hash = get_empty_hash(hasher, height - 1);
        merge_poseidon_hash_with_hasher(hasher, [Some(child_hash), Some(child_hash)], height)
    }
}
