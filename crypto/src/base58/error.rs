// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error decoding base58 string")]
    Base58DecodeError(#[from] bs58::decode::Error),

    #[error("Other error: '{0}'")]
    OtherError(String),
}
