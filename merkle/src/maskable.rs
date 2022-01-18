// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

/// A merkle tree that can be masked by [super::MaskingMerkleTree]
pub trait MaskableMerkleTree {
    /// Register a [super::MaskingMerkleTree]
    fn register(&self, mask: impl MaskingMerkleTree) -> bool;
    /// Unregister a [super::MaskingMerkleTree]
    fn unregister(&self, mask: impl MaskingMerkleTree) -> bool;
}
