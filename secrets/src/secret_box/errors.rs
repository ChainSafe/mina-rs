// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use proof_systems::mina_signer::keypair::KeypairError;

/// Error type of handling wallet files generated from mina-keypair-gen tool
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Invalid secret box primitive
    #[error("Invalid secret box primitive")]
    InvalidSecretBoxPrimitiveError,

    /// Invalid password hash primitive
    #[error("Invalid password hash primitive")]
    InvalidPasswordHashPrimitiveError,

    /// Fail to serialize / deserialize json
    #[error("Fail to serialize / deserialize json: {0}")]
    JsonSerdeError(#[from] serde_json::Error),

    /// Fail to decode base58 string
    #[error("Fail to decode base58 string: {0}")]
    Base58DecodeError(#[from] bs58::decode::Error),

    /// Argon2Error
    #[error("Argon2Error: {0}")]
    Argon2Error(String),

    /// PasswordHashError
    #[error("PasswordHashError: {0}")]
    PasswordHashError(String),

    /// AeadError
    #[error("AeadError: {0}")]
    AeadError(String),

    /// KeypairError
    #[error("KeypairError: {0}")]
    KeypairError(KeypairError),
}
