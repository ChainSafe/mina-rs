// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account based permissions

use crate::from_graphql_json::FromGraphQLJson;
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{bitvec::prelude::BitVec, ChunkedROInput, ToChunkedROInput};
use std::str::FromStr;
use strum::EnumString;

/// The level of auth required to perform a particular action with an account
#[derive(Clone, Debug, EnumString, AutoFrom)]
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

impl ToChunkedROInput for Permissions {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.edit_state)
            .append_chunked(&self.send)
            .append_chunked(&self.receive)
            .append_chunked(&self.set_delegate)
            .append_chunked(&self.set_permissions)
            .append_chunked(&self.set_verification_key)
            .append_chunked(&self.set_zkapp_uri)
            .append_chunked(&self.edit_sequence_state)
            .append_chunked(&self.set_token_symbol)
            .append_chunked(&self.increment_nonce)
            .append_chunked(&self.set_voting_for)
    }
}

impl FromGraphQLJson for Permissions {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            edit_state: AuthRequired::from_str(json["editState"].as_str().unwrap_or_default())?,
            send: AuthRequired::from_str(json["send"].as_str().unwrap_or_default())?,
            receive: AuthRequired::from_str(json["receive"].as_str().unwrap_or_default())?,
            set_delegate: AuthRequired::from_str(json["setDelegate"].as_str().unwrap_or_default())?,
            set_permissions: AuthRequired::from_str(
                json["setPermissions"].as_str().unwrap_or_default(),
            )?,
            set_verification_key: AuthRequired::from_str(
                json["setVerificationKey"].as_str().unwrap_or_default(),
            )?,
            set_zkapp_uri: AuthRequired::from_str(
                json["setZkappUri"].as_str().unwrap_or_default(),
            )?,
            edit_sequence_state: AuthRequired::from_str(
                json["editSequenceState"].as_str().unwrap_or_default(),
            )?,
            set_token_symbol: AuthRequired::from_str(
                json["setTokenSymbol"].as_str().unwrap_or_default(),
            )?,
            increment_nonce: AuthRequired::from_str(
                json["editState"].as_str().unwrap_or_default(),
            )?,
            set_voting_for: AuthRequired::from_str(
                json["setVotingFor"].as_str().unwrap_or_default(),
            )?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissions_from_graphql_json() -> anyhow::Result<()> {
        const JSON_STR: &str = r###"
        {
            "editSequenceState": "Signature",
            "editState": "Signature",
            "incrementNonce": "Signature",
            "receive": "None",
            "send": "Signature",
            "setDelegate": "Signature",
            "setPermissions": "Signature",
            "setTokenSymbol": "Signature",
            "setVerificationKey": "Signature",
            "setVotingFor": "Signature",
            "setZkappUri": "Signature"
          }
        "###;
        let json = serde_json::from_str(JSON_STR)?;
        _ = Permissions::from_graphql_json(&json)?;
        Ok(())
    }
}
