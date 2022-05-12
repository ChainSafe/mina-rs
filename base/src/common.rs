// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Common data types.
//!

use mina_serialization_types_macros::AutoFrom;

/// Wrapper of Vec<u8>
#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::common::ByteVec)]
pub struct ByteVec(pub Vec<u8>);
