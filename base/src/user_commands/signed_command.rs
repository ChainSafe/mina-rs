// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Signed commands are commands that require signing with some accounts private key

use crate::numbers::{AccountNonce, Amount, GlobalSlotNumber, TokenId};
use crate::user_commands::memo::SignedCommandMemo;
use crate::user_commands::payment::PaymentPayload;

use proof_systems::mina_hasher::{Hashable, ROInput};
use proof_systems::mina_signer::{CompressedPubKey, NetworkId, Signature};

const TAG_BITS: usize = 3;
const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false, false, false];

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

impl Hashable for SignedCommandPayload {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        match &self.body {
            SignedCommandPayloadBody::PaymentPayload(pp) => {
                roi.append_field(self.common.fee_payer_pk.x);
                roi.append_field(pp.source_pk.x);
                roi.append_field(pp.receiver_pk.x);

                roi.append_u64(self.common.fee.0);
                roi.append_u64(self.common.fee_token.0);
                roi.append_bool(self.common.fee_payer_pk.is_odd);
                roi.append_u32(self.common.nonce.0);
                roi.append_u32(self.common.valid_until.0);
                roi.append_bytes(&self.common.memo.0);

                for tag_bit in PAYMENT_TX_TAG {
                    roi.append_bool(tag_bit);
                }

                roi.append_bool(pp.source_pk.is_odd);
                roi.append_bool(pp.receiver_pk.is_odd);
                roi.append_u64(pp.token_id.0);
                roi.append_u64(pp.amount.0);
                roi.append_bool(false); // this is the token locked field. Not sure where this belongs yet
            }
        };
        roi
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
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
    pub nonce: AccountNonce,
    /// UNIX timestamp after which the signed command is no longer valid
    pub valid_until: GlobalSlotNumber,
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
