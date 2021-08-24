// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use self::traits::S;
use fp_core::hkt::HKT;
pub mod traits;

pub enum SElemType<E> {
    Left(E),
    Right(E),
}

pub trait Hash<H>: HKT<H> {
    fn merge<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(Self::Current, usize, H, H) -> H,
        H: std::hash::Hash;
}

pub struct Make<U: Hash<H>, H> {
    _a: PhantomData<U>,
    _b: PhantomData<H>,
}

impl<U: Hash<H>, H> S for Make<U, H>
where
    H: std::hash::Hash + Eq,
{
    type Hash = H;

    type Elem = SElemType<Self::Hash>;

    fn elem_hash(e: Self::Elem) -> Self::Hash {
        match e {
            SElemType::Left(e) => e,
            SElemType::Right(e) => e,
        }
    }

    fn implied_root(t: &[Self::Elem], leaf_hash: Self::Hash) -> Self::Hash {
        todo!()
    }

    fn check_path(t: &[Self::Elem], leaf_hash: Self::Hash, root: Self::Hash) -> bool {
        Self::implied_root(t, leaf_hash) == root
    }
}
