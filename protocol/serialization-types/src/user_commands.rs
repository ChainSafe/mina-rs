// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::signatures::{PublicKey2V1, PublicKeyV1, SignatureV1};
use crate::v1::{AmountV1, ExtendedU32, ExtendedU64_3};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum UserCommand {
    SignedCommand(SignedCommandV1),
    // FIXME: other variants are not covered by current test block
}

pub type UserCommandV1 = Versioned<Versioned<UserCommand, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommand {
    pub payload: SignedCommandPayloadV1,
    pub signer: PublicKey2V1,
    pub signature: SignatureV1,
}

pub type SignedCommandV1 = Versioned<Versioned<SignedCommand, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommandPayload {
    pub common: SignedCommandPayloadCommonV1,
    pub body: SignedCommandPayloadBodyV1,
}

pub type SignedCommandPayloadV1 = Versioned<Versioned<SignedCommandPayload, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommandPayloadCommon {
    pub fee: AmountV1,
    pub fee_token: SignedCommandFeeTokenV1,
    pub fee_payer_pk: PublicKeyV1,
    pub nonce: ExtendedU32,
    pub valid_until: ExtendedU32,
    pub memo: SignedCommandMemoV1,
}

pub type SignedCommandPayloadCommonV1 =
    Versioned<Versioned<Versioned<SignedCommandPayloadCommon, 1>, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum SignedCommandPayloadBody {
    PaymentPayload(PaymentPayloadV1),
    // FIXME: other variants are not covered by current test block
}

pub type SignedCommandPayloadBodyV1 = Versioned<Versioned<SignedCommandPayloadBody, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PaymentPayload {
    pub source_pk: PublicKeyV1,
    pub receiver_pk: PublicKeyV1,
    pub token_id: ExtendedU64_3,
    pub amount: AmountV1,
}

pub type PaymentPayloadV1 = Versioned<Versioned<PaymentPayload, 1>, 1>;

pub type SignedCommandFeeTokenV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;

pub type SignedCommandMemoV1 = Versioned<Vec<u8>, 1>;

// FIXME: No test coverage yet
pub type SnappCommand = Versioned<Versioned<(), 1>, 1>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionJson {
    pub public_key: String,
    pub signature: SignatureJson,
    pub payload: PayloadJson,
}

#[derive(Debug, Serialize)]
pub struct SignatureJson {
    pub field: String,
    pub scalar: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadJson {
    pub to: String,
    pub from: String,
    pub fee: String,
    pub amount: String,
    pub nonce: String,
    pub memo: String,
    pub valid_until: String,
}
