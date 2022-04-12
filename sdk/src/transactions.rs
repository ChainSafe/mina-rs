use serde::Serialize;

pub use mina_rs_base::user_commands::signed_command::builder::SignedTransferCommandBuilder;
pub use mina_rs_base::user_commands::signed_command::{SignedCommand, SignedCommandPayloadBody};
use num_bigint::BigUint;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TransactionJson {
    public_key: String,
    signature: SignatureJson,
    payload: PayloadJson,
}

#[derive(Debug, Serialize)]
struct SignatureJson {
    field: String,
    scalar: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PayloadJson {
    to: String,
    from: String,
    fee: String,
    amount: String,
    nonce: String,
    memo: String,
    valid_until: String,
}

trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl ToJsonString for SignedCommand {
    fn to_json_string(&self) -> String {
        let common = &self.payload.common;
        let payload_json = match &self.payload.body {
            SignedCommandPayloadBody::PaymentPayload(payload) => PayloadJson {
                to: payload.receiver_pk.into_address(),
                from: payload.source_pk.into_address(),
                fee: common.fee.0.to_string(),
                amount: payload.amount.0.to_string(),
                nonce: common.nonce.to_string(),
                memo: common.memo.to_string(),
                valid_until: common.valid_until.to_string(),
            },
            _ => unimplemented!(),
        };
        let signature_json = SignatureJson {
            field: Into::<BigUint>::into(self.signature.rx.0).to_str_radix(10),
            scalar: Into::<BigUint>::into(self.signature.s.0).to_str_radix(10),
        };
        let tx = TransactionJson {
            public_key: payload_json.from.clone(),
            signature: signature_json,
            payload: payload_json,
        };
        serde_json::ser::to_string_pretty(&tx).unwrap()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use mina_rs_base::user_commands::memo::SignedCommandMemo;
	use proof_systems::mina_signer::{Keypair, CompressedPubKey, NetworkId};

	macro_rules! make_signed_tx {
        ($sec_key:expr, $source_address:expr, $receiver_address:expr, $amount:expr, $fee:expr,
         $nonce:expr, $valid_until:expr, $memo:expr) => {{
            let kp = Keypair::from_hex($sec_key).expect("failed to create keypair");

            SignedTransferCommandBuilder::new(
                CompressedPubKey::from_address($source_address).expect("invalid source address"),
                CompressedPubKey::from_address($receiver_address)
                    .expect("invalid receiver address"),
                $amount,
                $fee,
                $nonce,
            )
            .valid_until($valid_until)
            .memo(SignedCommandMemo::try_from($memo).expect("invalid memo string"))
            .build()
			.into_signed_command(kp, NetworkId::TESTNET)
        }}
    }

	#[test]
	fn smoke() {
		let tx = make_signed_tx!(
            /* sender secret key  */ "164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718",
            /* source address     */ "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            /* receiver address   */ "B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt",
            /* amount             */ 1729000000000_u64,
            /* fee                */ 2000000000_u32,
            /* nonce              */ 16_u32,
            /* valid until        */ 271828_u32,
            /* memo               */ "Hello Mina!"
        );

        println!("{}", tx.to_json_string())
	}
}