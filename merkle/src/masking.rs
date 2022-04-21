// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// A merkle tree that can be used to mask [super::MaskableMerkleTree]
pub trait MaskingMerkleTree {
    /// Update a [super::MaskingMerkleTree] with changed notified from the [super::MaskableMerkleTree] it's registered in
    fn update(&self);
    /// Commits changes from a [super::MaskingMerkleTree] to the [super::MaskableMerkleTree] it's registered in
    fn commit(self, parent: impl MaskableMerkleTree);
}
