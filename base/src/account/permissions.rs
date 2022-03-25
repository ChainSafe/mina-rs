// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account based permissions

/// The level of auth required to perform a particular action with an account
#[derive(Clone, Debug)]
pub enum AuthRequired {
    /// None required
    None,
    /// Either a proof or a signature
    Either,
    /// Proof must be provided
    Proof,
    /// Signature must be provided
    Signature,
    /// Both proof and signature must be provided
    Both,
    /// This action can never occur
    Impossible,
}

/// Permissions associated with the account
#[derive(Clone, Debug)]
pub struct Permissions {
    /// If the account can stake
    pub stake: bool,
    /// Permission required to edit state
    pub edit_state: AuthRequired,
    /// Permission required to send a balance
    pub send: AuthRequired,
    /// Permission required to receive balance
    pub receive: AuthRequired,
    /// Permission required to set the delegate
    pub set_delegate: AuthRequired,
    /// Permission required to cange permissions
    pub set_permissions: AuthRequired,
    /// Permission require to set verification key
    pub set_verification_key: AuthRequired,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            stake: true,
            edit_state: AuthRequired::Signature,
            send: AuthRequired::Signature,
            receive: AuthRequired::None,
            set_delegate: AuthRequired::Signature,
            set_permissions: AuthRequired::Signature,
            set_verification_key: AuthRequired::Signature,
        }
    }
}
