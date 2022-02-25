// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_signer::ROInput;

/// Data structure that represents an account
/// <https://github.com/MinaProtocol/mina/blob/aa5f4efa5868b5b9f067d70e6a3f795d43dbb472/src/lib/mina_base/account.ml#L88>
#[derive(Debug, Clone, Copy)]
pub struct Account {
    public_key: i32,
    token_id: i32,
    token_permissions: i32,
    balance: i32,
    nonce: i32,
    receipt_chain_hash: i32,
    delegate: i32,
    voting_for: i32,
    timing: i32,
    permissions: i32,
    snapp: i32,
}

impl mina_signer::Hashable for Account {
    /// For details on ROInput construction logic, refer to
    /// <https://github.com/MinaProtocol/mina/blob/aa5f4efa5868b5b9f067d70e6a3f795d43dbb472/src/lib/mina_base/account.ml#L418>
    /// <https://github.com/MinaProtocol/mina/blob/aa5f4efa5868b5b9f067d70e6a3f795d43dbb472/src/lib/mina_base/account.ml#L391>
    /// <https://github.com/MinaProtocol/mina/blob/aa5f4efa5868b5b9f067d70e6a3f795d43dbb472/src/lib/mina_base/account.ml#L259>
    fn to_roinput(self) -> ROInput {
        let roi = ROInput::new();
        roi
    }
}
