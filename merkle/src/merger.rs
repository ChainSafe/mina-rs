// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait MerkleMerger {
    type Hash;
    fn merge(left: &Option<Self::Hash>, right: &Option<Self::Hash>) -> Option<Self::Hash>;
}
