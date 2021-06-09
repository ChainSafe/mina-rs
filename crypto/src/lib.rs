// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// Need to supress this warning for the moment so we can use the #[verson(x)]
// attribute macro. It seems we are in an awkward inbetween phase where it can't be
// defined above a derive macro (or the following warning) and it can't be defined
// below or it will error. The error will be fixed in the future when derive becomes like a regular
// attribute macro and an order of operations well defined
#![allow(legacy_derive_helpers)]

pub mod base58;
pub mod hash;
pub mod keys;
