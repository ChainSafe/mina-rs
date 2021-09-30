// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Error objects and codes

use std::fmt;
use std::io;
use std::result;

use serde::{de, ser};
use thiserror::Error;

/// A result whose error type is `Error`.
pub type Result<A> = result::Result<A, Error>;

#[derive(Debug, Error)]
pub enum Error {
    /// Error in underlying IO
    #[error("io error")]
    Io(#[from] io::Error),

    /// The end of stream was reached unexpectedly.
    #[error("end of stream")]
    EndOfStream,

    /// Size not given when serializing a sequence
    #[error("Attempting to serialize a sequence but size not provided")]
    SeqSizeNotProvided,

    /// Size not given when serializing a sequence
    #[error("Attempting to serialize a map but size not provided")]
    MapSizeNotProvided,

    /// Invalid byte
    #[error(
        "Invalid byte for deserializing a {dtype}. Expected one of: {allowed:?}, found: {byte}"
    )]
    InvalidByte {
        byte: u8,
        dtype: String,
        allowed: Vec<u8>,
    },

    /// Invalid utf-8 char
    #[error("Invalid byte sequence when attempting to deserialize utf-8 char: {bytes:?}")]
    InvalidUtf8 { bytes: Vec<u8> },

    #[error("Invalid byte when deserializing option. First byte must be 0 or 1, got {got}")]
    InvalidOptionByte { got: u8 },

    /// Invalid integer prefix byte
    #[error("Invalid byte when deserializing an integer. First byte must be a size flag or a value < 0x80")]
    InvalidIntegerByte { byte: u8 },

    /// Destination integer type too small
    #[error("Attempted to deserialize an integer into a desgination type that is too small")]
    DestinationIntegerOverflow,

    /// Error occurred at a given position (recursive variant) [not currently used]
    #[error("Error: {error}, at position: {pos}")]
    ErrorAt {
        #[source]
        error: Box<Error>,
        pos: usize,
    },

    /// Functionality will not be implemented. Probably it does not make sense for this format
    #[error(
        "Functionality will not be implemented. Probably it does not make sense for this format"
    )]
    WontImplement,

    /////// Loose deserialization specific errors
    /// Error traversing the layout for loose deserialization
    #[error("Layout traversal error")]
    LayoutTraversalError(#[from] crate::value::layout::traverse::Error),

    #[error("Unexpected end of layout")]
    UnexpectedEndOfLayout,

    #[error("Layout iterator error")]
    LayoutIteratorError,

    #[error("Attempting to deserialize into a loose type without a layout")]
    DeserializingLooseTypeWithoutLayout,

    #[error("Unknown custom type {typ}")]
    UnknownCustomType { typ: String },

    /// There is no logic implemented to deserialize this rule yet
    #[error("Unimplemented rule")]
    UnimplementedRule,

    //////////////////////////////////
    /// Some user-defined error occurred.
    #[error("{message}")]
    Custom {
        /// The user-defined error message.
        message: String,
    },
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom {
            message: msg.to_string(),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom {
            message: msg.to_string(),
        }
    }
}
