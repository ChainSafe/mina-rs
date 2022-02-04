// Copyright 2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! types and functions related to Mina signed_command

use crate::minting_payload::MintingPayload;
use crate::new_account_payload::NewAccountPayload;
use crate::new_token_payload::NewTokenPayload;
use crate::numbers::{Amount, ExtendedU32, ExtendedU64_3};
use crate::stake_delegation::StakeDelegation;
use mina_crypto::signature::{PublicKey2, PublicKey3, Signature};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// Commands for signed transaction
pub struct SignedCommand {
    /// The payload is the part of transmitted data that is the actual intended message
    pub payload: SignedCommandPayload,
    /// Signer's public key for signing transactions on various networks for Mina Protocol
    pub signer: PublicKey3,
    /// The signature verifies in the block verifies and that verify consensus
    pub signature: Signature,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
///  The payload is the part of transmitted signed comand data that is the actual intended message
pub struct SignedCommandPayload {
    /// Payload common of a signed transaction Request
    pub common: SignedCommandPayloadCommon,
    ///  Payload body of a signed transaction Request
    pub body: SignedCommandPayloadBody,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
/// Verifies validity of the signed transaction.
pub struct SignedCommandPayloadCommon {
    /// The fee to be paid to the network to process the transaction
    pub fee: Amount,
    ///
    pub fee_token: SignedCommandFeeToken,
    ///
    pub fee_payer_pk: PublicKey2,
    /// The nonce in the Sender’s account
    pub nonce: ExtendedU32,
    ///
    pub valid_until: ExtendedU32,
    ///
    pub memo: SignedCommandMemo,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
#[non_exhaustive]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/signed_command_payload.ml#L200
/// payload body of a signed transaction command HTTP Request
pub enum SignedCommandPayloadBody {
    /// The part of transmitted payment data that is the actual intended message
    PaymentPayload(PaymentPayload),
    /// The part of transmitted stake delegation data that is the actual intended message
    StakeDelegationPayload(StakeDelegation),
    /// The part of transmitted create new token data that is the actual intended message
    CreateNewTokenPayload(NewTokenPayload),
    /// The part of transmitted create token account data that is the actual intended message
    CreateTokenAccountPayload(NewAccountPayload),
    /// The part of transmitted Mint tokens data that is the actual intended message
    MintTokensPayload(MintingPayload),
}

impl Default for SignedCommandPayloadBody {
    fn default() -> Self {
        Self::PaymentPayload(PaymentPayload::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// The part of transmitted payment data that is the actual intended message
pub struct PaymentPayload {
    /// The source public key
    pub source_pk: PublicKey2,
    /// The public key of the intended recipient
    pub receiver_pk: PublicKey2,
    /// Token is implemented as ERC20 smart contract
    pub token_id: ExtendedU64_3,
    /// The amount of mina you are sending
    pub amount: Amount,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
///
pub struct SignedCommandFeeToken(pub u64);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
///
pub struct SignedCommandMemo(pub Vec<u8>);

impl TryFrom<&str> for SignedCommandMemo {
    type Error = SignedCommandMemoError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        const DIGEST_LEN: usize = 32;
        const MAX_INPUT_STRING_LENGTH: usize = DIGEST_LEN;
        const MEMO_LEN: usize = DIGEST_LEN + 2;
        const TAG_INDEX: usize = 0;
        // const DIGEST_TAG: u8 = 0;
        const LEN_INDEX: usize = 1;
        const BYTES_TAG: u8 = 1;
        if s.len() > MAX_INPUT_STRING_LENGTH {
            return Err(SignedCommandMemoError::StringTooLong);
        }
        let mut v = vec![0; MEMO_LEN];
        v[TAG_INDEX] = BYTES_TAG;
        v[LEN_INDEX] = s.len() as u8;
        for (i, b) in s.as_bytes().iter().enumerate() {
            v[i + 2] = *b;
        }
        Ok(Self(v))
    }
}

impl TryFrom<String> for SignedCommandMemo {
    type Error = SignedCommandMemoError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

#[derive(Debug, Error)]
///
pub enum SignedCommandMemoError {
    #[error("Input string is too long")]
    ///
    StringTooLong,
}
