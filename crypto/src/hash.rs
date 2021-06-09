// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! A trait that any type can implement to be able to produce a valid Mina hash
//!

pub trait MinaHash {
	type HashType;

	fn prefix() -> &str {
		
	}
}

