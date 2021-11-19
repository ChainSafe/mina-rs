// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error reading snark key file")]
    ReadError(#[from] std::io::Error),

    #[error("Error deserializing header JSON")]
    HeaderDeserializationError(#[from] serde_json::Error),

    #[error("Expected first line to be 'MINA_SNARK_KEYS', got '{0}'")]
    FileIdError(String),

    #[error("Reached the end of the file")]
    UnexpectedEndOfFileError,

    #[error("Error deserializing bin-prot encoded body")]
    BinProtDeserializationError(#[from] bin_prot::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
