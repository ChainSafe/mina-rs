// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::{
    account::{TimedData, TimedDataV1},
    v1::*,
};
use versioned::Versioned;

impl From<Account> for AccountV1 {
    fn from(t: Account) -> Self {
        AccountV1::new(Versioned::new(Versioned::new(
            mina_serialization_types::account::Account {
                public_key: t.public_key.into(),
                token_id: t.token_id.into(),
                token_permissions: t.token_permissions.into(),
                balance: t.balance.into(),
                nonce: t.nonce.into(),
                receipt_chain_hash: t.receipt_chain_hash.into_inner().into(),
                delegate: t.delegate.map(Into::into),
                voting_for: t.voting_for.into(),
                timing: t.timing.into(),
                permissions: t.permissions.into(),
                snapp: t.snapp.map(Into::into),
            },
        )))
    }
}
impl From<AccountV1> for Account {
    fn from(t: AccountV1) -> Self {
        let t = t.t.t.t;
        Self {
            public_key: t.public_key.into(),
            token_id: t.token_id.t.t.t.into(),
            token_permissions: t.token_permissions.into(),
            balance: t.balance.t.t.into(),
            nonce: t.nonce.t.t.into(),
            receipt_chain_hash: t.receipt_chain_hash.into(),
            delegate: t.delegate.map(Into::into),
            voting_for: t.voting_for.into(),
            timing: t.timing.into(),
            permissions: t.permissions.into(),
            snapp: t.snapp.map(Into::into),
        }
    }
}

impl From<TokenPermissions> for TokenPermissionsV1 {
    fn from(t: TokenPermissions) -> Self {
        use mina_serialization_types::account::TokenPermissions as TP;
        match t {
            TokenPermissions::TokenOwned {
                disable_new_accounts,
            } => Self::new(Versioned::new(TP::TokenOwned {
                disable_new_accounts,
            })),
            TokenPermissions::NotOwned { account_disabled } => {
                Self::new(Versioned::new(TP::NotOwned { account_disabled }))
            }
        }
    }
}
impl From<TokenPermissionsV1> for TokenPermissions {
    fn from(t: TokenPermissionsV1) -> Self {
        use mina_serialization_types::account::TokenPermissions as TP;
        match t.t.t {
            TP::TokenOwned {
                disable_new_accounts,
            } => TokenPermissions::TokenOwned {
                disable_new_accounts,
            },
            TP::NotOwned { account_disabled } => TokenPermissions::NotOwned { account_disabled },
        }
    }
}

impl From<Timing> for TimingV1 {
    fn from(t: Timing) -> Self {
        use mina_serialization_types::account::Timing as TP;
        match t {
            Timing::Untimed => Self::new(Versioned::new(TP::Untimed)),
            Timing::Timed {
                initial_minimum_balance,
                cliff_time,
                cliff_amount,
                vesting_increment,
                vesting_period,
            } => Self::new(Versioned::new(TP::Timed(TimedDataV1::new(TimedData {
                initial_minimum_balance: initial_minimum_balance.into(),
                cliff_time: cliff_time.into(),
                cliff_amount: cliff_amount.into(),
                vesting_increment: vesting_increment.into(),
                vesting_period: vesting_period.into(),
            })))),
        }
    }
}
impl From<TimingV1> for Timing {
    fn from(t: TimingV1) -> Self {
        use mina_serialization_types::account::Timing as T;
        match t.t.t {
            T::Untimed => Timing::Untimed,
            T::Timed(Versioned {
                t:
                    TimedData {
                        initial_minimum_balance,
                        cliff_time,
                        cliff_amount,
                        vesting_increment,
                        vesting_period,
                    },
                ..
            }) => Timing::Timed {
                initial_minimum_balance: initial_minimum_balance.t.t.into(),
                cliff_time: cliff_time.into(),
                cliff_amount: cliff_amount.t.t.into(),
                vesting_increment: vesting_increment.t.t.into(),
                vesting_period: vesting_period.into(),
            },
        }
    }
}

impl From<Permissions> for PermissionsV1 {
    fn from(t: Permissions) -> Self {
        PermissionsV1::new(Versioned::new(
            mina_serialization_types::account::Permissions {
                stake: t.stake,
                edit_state: t.edit_state.into(),
                send: t.send.into(),
                receive: t.receive.into(),
                set_delegate: t.set_delegate.into(),
                set_permissions: t.set_permissions.into(),
                set_verification_key: t.set_verification_key.into(),
            },
        ))
    }
}
impl From<PermissionsV1> for Permissions {
    fn from(t: PermissionsV1) -> Self {
        Self {
            stake: t.t.t.stake,
            edit_state: t.t.t.edit_state.into(),
            send: t.t.t.send.into(),
            receive: t.t.t.receive.into(),
            set_delegate: t.t.t.set_delegate.into(),
            set_permissions: t.t.t.set_permissions.into(),
            set_verification_key: t.t.t.set_verification_key.into(),
        }
    }
}

impl From<AuthRequired> for AuthRequiredV1 {
    fn from(t: AuthRequired) -> Self {
        use mina_serialization_types::account::AuthRequired as AR;
        match t {
            AuthRequired::None => Self::new(AR::None),
            AuthRequired::Either => Self::new(AR::Either),
            AuthRequired::Proof => Self::new(AR::Proof),
            AuthRequired::Signature => Self::new(AR::Signature),
            AuthRequired::Both => Self::new(AR::Both),
            AuthRequired::Impossible => Self::new(AR::Impossible),
        }
    }
}
impl From<AuthRequiredV1> for AuthRequired {
    fn from(t: AuthRequiredV1) -> Self {
        use mina_serialization_types::account::AuthRequired as AR;
        match t.t {
            AR::None => AuthRequired::None,
            AR::Either => AuthRequired::Either,
            AR::Proof => AuthRequired::Proof,
            AR::Signature => AuthRequired::Signature,
            AR::Both => AuthRequired::Both,
            AR::Impossible => AuthRequired::Impossible,
        }
    }
}
