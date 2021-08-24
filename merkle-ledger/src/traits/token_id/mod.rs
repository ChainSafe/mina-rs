// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod stable;

pub trait TokenId {
    type T: stable::latest::K;
    fn default() -> Self::T;
    fn next(other: &Self::T) -> Self::T;
}
