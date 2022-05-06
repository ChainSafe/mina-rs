// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::user_commands::{SignedCommand, SignedCommandPayloadBody};
use mina_serialization_types::json::{PayloadJson, SignatureJson, TransactionJson};
use num_bigint::BigUint;

impl From<SignedCommand> for TransactionJson {
    fn from(t: SignedCommand) -> Self {
        let common = &t.payload.common;
        let payload_json = match &t.payload.body {
            SignedCommandPayloadBody::PaymentPayload(payload) => PayloadJson {
                to: payload.receiver_pk.into_address(),
                from: payload.source_pk.into_address(),
                fee: common.fee.0.to_string(),
                amount: payload.amount.0.to_string(),
                nonce: common.nonce.to_string(),
                memo: common.memo.to_string(),
                valid_until: common.valid_until.to_string(),
            },
        };
        let signature_json = SignatureJson {
            field: Into::<BigUint>::into(t.signature.rx.0).to_str_radix(10),
            scalar: Into::<BigUint>::into(t.signature.s.0).to_str_radix(10),
        };
        TransactionJson {
            public_key: payload_json.from.clone(),
            signature: signature_json,
            payload: payload_json,
        }
    }
}

impl From<TransactionJson> for SignedCommand {
    fn from(_t: TransactionJson) -> Self {
        todo!()
    }
}
