// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account based permissions

use mina_serialization_types_macros::AutoFrom;

/// The level of auth required to perform a particular action with an account
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::AuthRequired)]
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
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::Permissions)]
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

/// Permissions associated with the account
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::PermissionsHardFork)]
pub struct PermissionsHardFork {
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
    /// Permission require to set zkapp uri
    pub set_zkapp_uri: AuthRequired,
    /// Permission require to edit sequence state
    pub edit_sequence_state: AuthRequired,
    /// Permission require to set token symbol
    pub set_token_symbol: AuthRequired,
    /// Permission require to increment nonce
    pub increment_nonce: AuthRequired,
    /// Permission require to set voting for
    pub set_voting_for: AuthRequired,
}
