// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account based permissions

use mina_serialization_types_macros::AutoFrom;

use proof_systems::mina_hasher::{Hashable, ROInput};

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

// Auxiliary structure for encoding the
#[derive(Clone, Copy)]
struct AuthRequiredEncoding {
    constant: bool,
    signature_necessary: bool,
    signature_sufficient: bool,
}

impl From<&AuthRequired> for AuthRequiredEncoding {
    fn from(auth_req: &AuthRequired) -> Self {
        match auth_req {
            AuthRequired::None => AuthRequiredEncoding {
                constant: true,
                signature_necessary: false,
                signature_sufficient: true,
            },
            AuthRequired::Either => AuthRequiredEncoding {
                constant: false,
                signature_necessary: false,
                signature_sufficient: true,
            },
            AuthRequired::Proof => AuthRequiredEncoding {
                constant: false,
                signature_necessary: false,
                signature_sufficient: false,
            },
            AuthRequired::Signature => AuthRequiredEncoding {
                constant: false,
                signature_necessary: true,
                signature_sufficient: true,
            },
            AuthRequired::Both => AuthRequiredEncoding {
                constant: false,
                signature_necessary: true,
                signature_sufficient: false,
            },
            AuthRequired::Impossible => AuthRequiredEncoding {
                constant: true,
                signature_necessary: true,
                signature_sufficient: false,
            },
        }
    }
}

impl Hashable for AuthRequiredEncoding {
    type D = ();

    fn to_roinput(&self) -> proof_systems::mina_hasher::ROInput {
        let mut roi = ROInput::new();

        roi.append_bool(self.constant);
        roi.append_bool(self.signature_necessary);
        roi.append_bool(self.signature_sufficient);

        roi
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        Some("CodaAuthRequired".to_string())
    }
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

impl Hashable for Permissions {
    type D = ();

    fn to_roinput(&self) -> proof_systems::mina_hasher::ROInput {
        let mut roi = ROInput::new();
        roi.append_bool(self.stake);
        let edit_state: AuthRequiredEncoding = (&self.edit_state).into();
        let send: AuthRequiredEncoding = (&self.send).into();
        let receive: AuthRequiredEncoding = (&self.receive).into();
        let set_delegate: AuthRequiredEncoding = (&self.set_delegate).into();
        let set_permissions: AuthRequiredEncoding = (&self.set_permissions).into();
        let set_verification_key: AuthRequiredEncoding = (&self.set_verification_key).into();
        roi.append_hashable(&edit_state);
        roi.append_hashable(&send);
        roi.append_hashable(&receive);
        roi.append_hashable(&set_delegate);
        roi.append_hashable(&set_permissions);
        roi.append_hashable(&set_verification_key);

        roi
    }

    fn domain_string(_domain_param: Self::D) -> Option<String> {
        Some("CodaPermissions".to_string())
    }
}
