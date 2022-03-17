// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account token permissions

/// Need to learn exactly what this is..
#[derive(Clone, Debug)]
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
