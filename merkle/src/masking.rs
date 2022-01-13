// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub trait MaskingMerkleTree {
    fn update(&self);
    fn commit(self, parent: impl MaskableMerkleTree);
}
