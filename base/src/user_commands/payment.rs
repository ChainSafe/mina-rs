use crate::numbers::{Amount, TokenId};
use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct PaymentPayload {
    pub source_pk: PublicKey,
    pub receiver_pk: PublicKey,
    pub token_id: TokenId,
    pub amount: Amount,
}
