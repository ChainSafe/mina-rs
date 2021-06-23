// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Length(u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Amount(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct GlobalSlot(u32);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct BlockTime(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct BlockTimeSpan(u64);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct ProtocolVersion {
	major: u32,
	minor: u32,
	patch: u32,
}


pub mod currency {

	use serde::{Deserialize, Serialize};
	use serde_versions_derive::version;

	#[version(1)]
	#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
	pub struct Amount(u64);

	#[version(1)]
	#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
	pub struct Fee(u64);

	#[version(1)]
	#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Hash)]
	pub struct Balance(u64);
}
