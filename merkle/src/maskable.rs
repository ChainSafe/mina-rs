// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub trait MaskableMerkleTree {
    fn register(&self, mask: impl MaskingMerkleTree) -> bool;
    fn unregister(&self, mask: impl MaskingMerkleTree) -> bool;
}
