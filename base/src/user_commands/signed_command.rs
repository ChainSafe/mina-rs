use crate::numbers::{Amount, ExtendedU32, TokenId};
use mina_crypto::signature::{PublicKey, Signature};
use serde::{Deserialize, Serialize};

use crate::user_commands::memo::SignedCommandMemo;
use crate::user_commands::payment::PaymentPayload;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommand {
    pub payload: SignedCommandPayload,
    pub signer: PublicKey,
    pub signature: Signature,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommandPayload {
    pub common: SignedCommandPayloadCommon,
    pub body: SignedCommandPayloadBody,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommandPayloadCommon {
    pub fee: Amount,
    pub fee_token: TokenId,
    pub fee_payer_pk: PublicKey,
    pub nonce: ExtendedU32,
    pub valid_until: ExtendedU32,
    pub memo: SignedCommandMemo,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum SignedCommandPayloadBody {
    PaymentPayload(PaymentPayload),
    // FIXME: other variants are not covered by current test block
}

impl Default for SignedCommandPayloadBody {
    fn default() -> Self {
        Self::PaymentPayload(PaymentPayload::default())
    }
}
