use proof_systems::mina_signer::keypair::KeypairError;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid secret box primitive")]
    InvalidSecretBoxPrimitiveError,

    #[error("Invalid password hash primitive")]
    InvalidPasswordHashPrimitiveError,

    #[error("Fail to serialize / deserialize json: {0}")]
    JsonSerdeError(#[from] serde_json::Error),

    #[error("Fail to decode base58 string: {0}")]
    Base58DecodeError(#[from] bs58::decode::Error),

    #[error("Argon2Error: {0}")]
    Argon2Error(String),

    #[error("PasswordHashError: {0}")]
    PasswordHashError(String),

    #[error("AeadError: {0}")]
    AeadError(String),

    #[error("KeypairError: {0}")]
    KeypairError(KeypairError),
}
