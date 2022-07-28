// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Common data types.
//!

use ark_ff::Zero;
use mina_serialization_types_macros::AutoFrom;
use once_cell::sync::OnceCell;
use proof_systems::{
    mina_hasher::{Fp, Hashable, ROInput},
    mina_signer::CompressedPubKey,
    ChunkedROInput, ToChunkedROInput,
};

/// Wrapper of Vec<u8>
#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::common::ByteVec)]
pub struct ByteVec(pub Vec<u8>);

/// Wrapper of [CompressedPubKey] that implements [Hashable]
#[derive(Debug, Clone)]
pub struct CompressedPubKeyHashableWrapper<'a>(pub &'a CompressedPubKey);

impl<'a> Default for CompressedPubKeyHashableWrapper<'a> {
    fn default() -> Self {
        static INSTANCE: OnceCell<CompressedPubKey> = OnceCell::new();
        Self(INSTANCE.get_or_init(|| CompressedPubKey {
            x: Fp::zero(),
            is_odd: false,
        }))
    }
}

impl<'a> Hashable for CompressedPubKeyHashableWrapper<'a> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_field(self.0.x);
        roi.append_bool(self.0.is_odd);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl<'a> ToChunkedROInput for CompressedPubKeyHashableWrapper<'a> {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_field(self.0.x)
            .append_bool(self.0.is_odd)
    }
}

/// Wrapper of [Option<CompressedPubKey>] that implements [Hashable]
#[derive(Debug, Clone)]
pub struct CompressedPubKeyOptionHashableWrapper<'a>(pub &'a Option<CompressedPubKey>);

impl<'a> Hashable for CompressedPubKeyOptionHashableWrapper<'a> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&if let Some(pk) = self.0 {
            CompressedPubKeyHashableWrapper(pk)
        } else {
            CompressedPubKeyHashableWrapper::default()
        });
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl<'a> ToChunkedROInput for CompressedPubKeyOptionHashableWrapper<'a> {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        if let Some(pk) = self.0 {
            CompressedPubKeyHashableWrapper(pk)
        } else {
            CompressedPubKeyHashableWrapper::default()
        }
        .to_chunked_roinput()
    }
}
