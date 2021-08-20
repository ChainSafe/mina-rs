// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde_json::Value;

/// Used as a helper to deserialize yojson style enums using Serde
/// In Yojson variants are encoded as a list where the first element is the name of the variant
/// and subsequent elements are the variant data.
///
/// To deserialize this to a Rust enum, tag the enum with `#[serde(try_from = "ListTaggedEnum")]`
/// and implement TryFrom<ListTaggedEnum> on the type
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ListTaggedEnum {
    None((String,)),
    One((String, Value)),
    Two((String, Value, Value)),
}
