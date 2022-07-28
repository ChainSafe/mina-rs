// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account token permissions

use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    bitvec::prelude::BitVec,
    mina_hasher::{Hashable, ROInput},
    ChunkedROInput, ToChunkedROInput,
};

/// FIXME: Need to learn exactly what this is..
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::TokenPermissions)]
pub enum TokenPermissions {
    /// Account does not own a token
    NotOwned {
        /// ?
        account_disabled: bool,
    },
    /// Account owns a token
    TokenOwned {
        /// ?
        disable_new_accounts: bool,
    },
}

impl Hashable for TokenPermissions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        match self {
            Self::NotOwned { account_disabled } => {
                roi.append_bool(false);
                roi.append_bool(*account_disabled);
            }
            Self::TokenOwned {
                disable_new_accounts,
            } => {
                roi.append_bool(true);
                roi.append_bool(*disable_new_accounts);
            }
        };
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for TokenPermissions {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        let mut bits = BitVec::with_capacity(2);
        match self {
            Self::NotOwned { account_disabled } => {
                bits.push(false);
                bits.push(*account_disabled);
            }
            Self::TokenOwned {
                disable_new_accounts,
            } => {
                bits.push(true);
                bits.push(*disable_new_accounts);
            }
        };
        let max_bits = bits.len() as u32;
        ChunkedROInput::new().append_packed(ChunkedROInput::bits_to_fp_unsafe(bits), max_bits)
    }
}
