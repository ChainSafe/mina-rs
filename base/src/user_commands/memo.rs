// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Commands can include memo fields which contain arbitrary byte data
//!

use mina_serialization_types_macros::AutoFrom;
use thiserror::Error;

/// A memo byte strong for a signed command
#[derive(Clone, Default, PartialEq, Debug, derive_more::From, derive_more::Into, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::SignedCommandMemo)]
pub struct SignedCommandMemo(pub Vec<u8>);

impl TryFrom<&str> for SignedCommandMemo {
    type Error = SignedCommandMemoError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        const DIGEST_LEN: usize = 32;
        const MAX_INPUT_STRING_LENGTH: usize = DIGEST_LEN;
        const MEMO_LEN: usize = DIGEST_LEN + 2;
        const TAG_INDEX: usize = 0;
        const LEN_INDEX: usize = 1;
        const BYTES_TAG: u8 = 1;
        if s.len() > MAX_INPUT_STRING_LENGTH {
            return Err(SignedCommandMemoError::StringTooLong);
        }
        let mut v = vec![0; MEMO_LEN];
        v[TAG_INDEX] = BYTES_TAG;
        v[LEN_INDEX] = s.len() as u8;
        for (i, b) in s.as_bytes().iter().enumerate() {
            v[i + 2] = *b;
        }
        Ok(Self(v))
    }
}

// TODO impl ToString for SignedCommandMemo

/// Error type for converting memo types
#[derive(Debug, Error)]
pub enum SignedCommandMemoError {
    /// Tried to build a memo from a string that is too long to fit
    #[error("Input string is too long")]
    StringTooLong,
}
