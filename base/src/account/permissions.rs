// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account based permissions

use ark_ff::BigInteger256;
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    bitvec::prelude::BitVec,
    mina_hasher::{Hashable, ROInput},
    ChunkedROInput, ToChunkedROInput,
};

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

impl ToChunkedROInput for AuthRequired {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        let mut roi = ChunkedROInput::new();
        let constant = matches!(self, Self::Impossible | Self::None);
        let signature_necessary = matches!(self, Self::Impossible | Self::Signature);
        let signature_sufficient = matches!(self, Self::Either | Self::Signature | Self::None);
        for b in [constant, signature_necessary, signature_sufficient] {
            let mut bits = BitVec::with_capacity(1);
            bits.push(b);
            roi = roi.append_packed(ChunkedROInput::bits_to_fp_unsafe(bits), 1);
        }
        roi
    }
}

/// Permissions associated with the account
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::PermissionsLegacy)]
pub struct PermissionsLegacy {
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
#[auto_from(mina_serialization_types::account::Permissions)]
pub struct Permissions {
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

impl Hashable for Permissions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        // FIXME: This is only for genesis ledger accounts
        roi.append_field(BigInteger256::from(3681400667).into());
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for Permissions {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append(self.set_voting_for.to_chunked_roinput())
            .append(self.increment_nonce.to_chunked_roinput())
            .append(self.set_token_symbol.to_chunked_roinput())
            .append(self.edit_sequence_state.to_chunked_roinput())
            .append(self.set_zkapp_uri.to_chunked_roinput())
            .append(self.set_verification_key.to_chunked_roinput())
            .append(self.set_permissions.to_chunked_roinput())
            .append(self.set_delegate.to_chunked_roinput())
            .append(self.receive.to_chunked_roinput())
            .append(self.send.to_chunked_roinput())
            .append(self.edit_state.to_chunked_roinput())
    }
}
