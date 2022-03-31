// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Signed commands are commands that require signing with some accounts private key

use crate::numbers::{Amount, ExtendedU32, TokenId};
use mina_crypto::signature::{Signature};

use mina_signer::CompressedPubKey;

use crate::user_commands::memo::SignedCommandMemo;
use crate::user_commands::payment::PaymentPayload;

/// Top level signed command type
#[derive(Clone, PartialEq, Debug)]
pub struct SignedCommand {
    /// The payload to sign
    pub payload: SignedCommandPayload,
    /// The signer (public key)
    pub signer: CompressedPubKey,
    /// The signature (result of signing payload with public key)
    pub signature: Signature,
}

/// The part of a signed command that needs to be serialized and signed
#[derive(Clone, PartialEq, Debug)]
pub struct SignedCommandPayload {
    /// Fields common to all command types
    pub common: SignedCommandPayloadCommon,
    /// Fields that depend on the type of command (e.g. payment, snapp, etc)
    pub body: SignedCommandPayloadBody,
}

/// Common fields required by all signed commands
#[derive(Clone, PartialEq, Debug)]
pub struct SignedCommandPayloadCommon {
    /// Amount paid in fees to include this command in a block
    pub fee: Amount,
    /// Token to be used to pay the fees
    pub fee_token: TokenId,
    /// The public key of the payer of the fees (need not be the signer)
    pub fee_payer_pk: CompressedPubKey,
    /// Nonce assicociated with account sending transaction
    pub nonce: ExtendedU32,
    /// UNIX timestamp after which the signed command is no longer valid
    pub valid_until: ExtendedU32,
    /// Arbitary bytes that can be included
    pub memo: SignedCommandMemo,
}

/// Enum of variable fields in a signed command
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum SignedCommandPayloadBody {
    /// Payment transfer fields
    PaymentPayload(PaymentPayload),
    // FIXME: other variants are not covered by current test block
}
