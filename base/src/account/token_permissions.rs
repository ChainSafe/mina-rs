// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account token permissions

use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_hasher::{Hashable, ROInput};

/// Need to learn exactly what this is..
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::TokenPermissions)]
pub enum TokenPermissions {
    /// Account owns a token
    TokenOwned {
        /// ?
        disable_new_accounts: bool,
    },
    /// Account does not own a token
    NotOwned {
        /// ?
        account_disabled: bool,
    },
}

impl Hashable for TokenPermissions {
    type D = ();

    fn to_roinput(&self) -> proof_systems::mina_hasher::ROInput {
        let mut roi = ROInput::new();
        match self {
            TokenPermissions::TokenOwned {
                disable_new_accounts,
            } => {
                roi.append_bool(true);
                roi.append_bool(*disable_new_accounts);
            }
            &TokenPermissions::NotOwned { account_disabled } => {
                roi.append_bool(false);
                roi.append_bool(account_disabled);
            }
        }
        roi
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        None
    }
}
