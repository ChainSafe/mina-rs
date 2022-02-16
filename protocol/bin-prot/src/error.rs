// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Error objects and codes

use std::fmt;
use std::fmt::Debug;
use std::io;
use std::result;

use serde::{de, ser};
use thiserror::Error;

/// A result whose error type is `Error`.
pub type Result<A> = result::Result<A, Error>;

#[derive(Debug, Error)]
/// Error type for BinProt serialization and deserialization
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

    /// Invalid byte encountered when deserializing
    #[error(
        "Invalid byte for deserializing a {dtype}. Expected one of: {allowed:?}, found: {byte}"
    )]
    InvalidByte {
        /// The byte that is invalid
        byte: u8,
        /// The data type that the deserializer is attempting to deserialize
        dtype: String,
        /// Bytes that are allowed in this context
        allowed: Vec<u8>,
    },

    /// Invalid utf-8 char
    #[error("Invalid byte sequence when attempting to deserialize utf-8 char: {bytes:?}")]
    InvalidUtf8 {
        /// Invalid byte sequence encountered
        bytes: Vec<u8>,
    },

    /// Invalid byte encountered deserializing option
    #[error("Invalid byte when deserializing option. First byte must be 0 or 1, got {got}")]
    InvalidOptionByte {
        /// Invalid byte encountered
        got: u8,
    },

    /// Invalid integer prefix byte
    #[error("Invalid byte when deserializing an integer. First byte must be a size flag or a value < 0x80")]
    InvalidIntegerByte {
        /// Invalid byte encountered
        byte: u8,
    },

    /// Encountered a variant index larger than allowed
    #[error("Invalid variant index detected. Currently only supports enums with < 256 variants")]
    VariantIndexTooLarge {
        /// Invalid index encountered
        index: u32,
    },

    /// Destination integer type too small
    #[error("Attempted to deserialize an integer into a desgination type that is too small")]
    DestinationIntegerOverflow,

    /// Functionality will not be implemented. Probably it does not make sense for this format
    #[error(
        "Functionality will not be implemented. Probably it does not make sense for this format"
    )]
    WontImplement,

    /// The layout ended before the reader is empty
    #[error("Unexpected end of layout")]
    UnexpectedEndOfLayout,

    /// Layout iterator errored, can be due to invalid layout or mismatch between layout and input
    #[error("Layout iterator error")]
    LayoutIteratorError,

    /// Have not provided a layout
    #[error("Attempting to deserialize into a loose type without a layout")]
    DeserializingLooseTypeWithoutLayout,

    /// An unknown custom type found in layout that deserializer code does not know
    /// not to handle
    #[error("Unknown custom type {typ}")]
    UnknownCustomType {
        /// The custom type identifier
        typ: String,
    },

    /// There is no logic implemented to deserialize this rule yet
    #[error("Unimplemented rule")]
    UnimplementedRule,

    /// When deserializing a polyvar the tag does not match any known tags for the type
    #[error("Unrecognised Polyvar tag {0}")]
    UnknownPolyvarTag(u32),

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
