use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IoError: {0}")]
    IoError(std::io::Error),
}
