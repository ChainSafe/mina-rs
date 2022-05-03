// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::{account::TimedData, v1::*};
use versioned::*;

impl From<TokenPermissions> for TokenPermissionsV1 {
    fn from(t: TokenPermissions) -> Self {
        use mina_serialization_types::account::TokenPermissions as TP;
        match t {
            TokenPermissions::TokenOwned {
                disable_new_accounts,
            } => TP::TokenOwned {
                disable_new_accounts,
            }
            .into(),
            TokenPermissions::NotOwned { account_disabled } => {
                TP::NotOwned { account_disabled }.into()
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
            Timing::Untimed => TP::Untimed.into(),
            Timing::Timed {
                initial_minimum_balance,
                cliff_time,
                cliff_amount,
                vesting_increment,
                vesting_period,
            } => TP::Timed(
                TimedData {
                    initial_minimum_balance: initial_minimum_balance.into(),
                    cliff_time: cliff_time.into(),
                    cliff_amount: cliff_amount.into(),
                    vesting_increment: vesting_increment.into(),
                    vesting_period: vesting_period.into(),
                }
                .into(),
            )
            .into(),
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
