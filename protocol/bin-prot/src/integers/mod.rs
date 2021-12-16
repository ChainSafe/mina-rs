// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! The variable length integer types used in BinProt
//! By default rust integer types are serialized using variable length integer. To force a specific type
//! annotate a field with `#[serde(with = "nat0")] or `#[serde(with = "integer")]` where nat0 and integer are 
//! the submodule exported from this module.

pub mod integer;
pub mod nat0;
