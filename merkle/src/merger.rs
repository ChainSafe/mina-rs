// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub trait MerkleMerger {
    type Hash;
    fn merge(
        left: &Option<(Self::Hash, MerkleTreeNodeMetadata)>,
        right: &Option<(Self::Hash, MerkleTreeNodeMetadata)>,
    ) -> Option<Self::Hash>;
}
