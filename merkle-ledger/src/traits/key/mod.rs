// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod stable;

pub trait Key {
    type T: stable::v1::K;
    fn empty() -> Self::T;
    fn to_string(other: &Self::T) -> String;
}
