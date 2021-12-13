// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}
