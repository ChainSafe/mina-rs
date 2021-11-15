use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Error reading snark key")]
    ReadError(#[from] std::io::Error),

	#[error("Error deserializing header JSON")]
    HeaderDeserializationError(#[from] serde_json::Error),

	#[error("Expected first line to be 'MINA_SNARK_KEYS', got '{0}'")]
    FileIdError(String),
	
	#[error("Reached the end of the file")]
    UnexpectedEndOfFileError,
}

pub type Result<T> = std::result::Result<T, Error>;
