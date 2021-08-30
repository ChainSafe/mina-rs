// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait Balance {
    type T: Eq + Into<usize>;
    fn zero() -> Self::T;
    fn to_int(&self) -> usize;
}
