// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait S: crate::mask::BaseMerkleTree {
    type UnattachedMask;
    type AttachedMask;
    fn register_mask(this: &Self::T, other: &Self::UnattachedMask) -> Self::AttachedMask;
    // fn unregister_mask_exn(grandchildren: TODO, loc: &str, attached_mask: &Self::Attached_mask) -> Self::Unattached;
}
