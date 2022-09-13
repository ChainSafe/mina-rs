// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Commands can include memo fields which contain arbitrary byte data
//!

use mina_serialization_types::{json::*, *};
use mina_serialization_types_macros::AutoFrom;
use thiserror::Error;

/// A memo byte strong for a signed command
#[derive(Clone, Default, Eq, PartialEq, Debug, derive_more::From, derive_more::Into, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::SignedCommandMemo)]
#[auto_from(mina_serialization_types::staged_ledger_diff::SignedCommandMemoJson)]
pub struct SignedCommandMemo(pub Vec<u8>);

impl_strconv_via_json!(SignedCommandMemo, SignedCommandMemoJson);

impl SignedCommandMemo {
    /// Try build from text
    pub fn try_from_text(s: impl AsRef<[u8]>) -> Result<Self, SignedCommandMemoError> {
        const DIGEST_LEN: usize = 32;
        const MAX_INPUT_STRING_LENGTH: usize = DIGEST_LEN;
        const MEMO_LEN: usize = DIGEST_LEN + 2;
        const TAG_INDEX: usize = 0;
        const LEN_INDEX: usize = 1;
        const BYTES_TAG: u8 = 1;
        let s = s.as_ref();
        if s.len() > MAX_INPUT_STRING_LENGTH {
            return Err(SignedCommandMemoError::StringTooLong);
        }
        let mut v = vec![0; MEMO_LEN];
        v[TAG_INDEX] = BYTES_TAG;
        v[LEN_INDEX] = s.len() as u8;
        for (i, &b) in s.iter().enumerate() {
            v[i + 2] = b;
        }
        Ok(Self(v))
    }

    /// Convert into text
    pub fn to_text(&self) -> String {
        let len = self.0[1] as usize;
        String::from_utf8_lossy(&self.0[2..(2 + len)]).into()
    }
}

/// Error type for converting memo types
#[derive(Debug, Error)]
pub enum SignedCommandMemoError {
    /// Tried to build a memo from a string that is too long to fit
    #[error("Input string is too long")]
    StringTooLong,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn memo_roundtrip() -> anyhow::Result<()> {
        let bs58_encoded = "E4Yd7qwaRCHR6t7i6ToM98eSUy5eKKadQUPZX7Vpw4CWBvWyd8fzK";
        let text_expected = "FPayment";
        let memo_from_bs58 = SignedCommandMemo::from_str(bs58_encoded)?;
        let memo_from_text = SignedCommandMemo::try_from_text(text_expected)?;
        assert_eq!(memo_from_bs58, memo_from_text);
        let memo = memo_from_bs58;
        assert_eq!(&memo.to_string(), bs58_encoded);
        assert_eq!(&memo.to_text(), text_expected);
        Ok(())
    }
}
