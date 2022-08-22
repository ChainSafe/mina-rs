// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Common data types.
//!

use mina_serialization_types_macros::AutoFrom;
use once_cell::sync::OnceCell;
use proof_systems::{mina_signer::CompressedPubKey, ChunkedROInput, ToChunkedROInput};

/// Wrapper of Vec<u8>
#[derive(Clone, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::common::ByteVec)]
pub struct ByteVec(pub Vec<u8>);

/// Wrapper of [CompressedPubKey] that implements [ToChunkedROInput]
#[derive(Debug, Clone)]
pub struct CompressedPubKeyHashableWrapper<'a>(pub &'a CompressedPubKey);

impl<'a> Default for CompressedPubKeyHashableWrapper<'a> {
    fn default() -> Self {
        static INSTANCE: OnceCell<CompressedPubKey> = OnceCell::new();
        Self(INSTANCE.get_or_init(CompressedPubKey::empty))
    }
}

impl<'a> ToChunkedROInput for CompressedPubKeyHashableWrapper<'a> {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_field(self.0.x)
            .append_bool(self.0.is_odd)
    }
}

/// Wrapper of [Option<CompressedPubKey>] that implements [ToChunkedROInput]
#[derive(Debug, Clone)]
pub struct CompressedPubKeyOptionHashableWrapper<'a>(pub &'a Option<CompressedPubKey>);

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
