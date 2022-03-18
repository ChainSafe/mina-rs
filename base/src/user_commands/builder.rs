//! Helpers for building a user command

use crate::numbers::{Amount, ExtendedU32};
use crate::user_commands::{
    PaymentPayload, SignedCommand, SignedCommandPayload, SignedCommandPayloadBody,
    SignedCommandPayloadCommon, UserCommand,
};
use mina_crypto::signature::PublicKey;

/// A builder for UserCommands to transfer funds
pub struct SignedTransferCommandBuilder {
    common: SignedCommandPayloadCommon,
    payment: PaymentPayload,
}

impl SignedTransferCommandBuilder {
    /// Create a new builder containing default transaction params
    pub fn new() -> Self {
        Self {
            common: SignedCommandPayloadCommon::default(),
            payment: PaymentPayload::default(),
        }
    }

    /// Set the payment recipient account
    pub fn to(self, receiver_pk: PublicKey) -> Self {
        Self {
            common: SignedCommandPayloadCommon { ..self.common },
            payment: PaymentPayload {
                receiver_pk,
                ..self.payment
            },
        }
    }

    /// Set the payment source account
    pub fn from(self, source_pk: PublicKey) -> Self {
        Self {
            common: SignedCommandPayloadCommon { ..self.common },
            payment: PaymentPayload {
                source_pk,
                ..self.payment
            },
        }
    }

    /// Set amount to transfer
    pub fn amount(self, amount: Amount) -> Self {
        Self {
            common: SignedCommandPayloadCommon { ..self.common },
            payment: PaymentPayload {
                amount,
                ..self.payment
            },
        }
    }

    /// Set the fee to pay the block producer
    pub fn fee(self, fee: Amount) -> Self {
        Self {
            common: SignedCommandPayloadCommon { fee, ..self.common },
            payment: PaymentPayload { ..self.payment },
        }
    }

    /// Set the nonce for the transaction
    pub fn nonce(self, nonce: ExtendedU32) -> Self {
        Self {
            common: SignedCommandPayloadCommon {
                nonce,
                ..self.common
            },
            payment: PaymentPayload { ..self.payment },
        }
    }

    // TODO: Add additional setters once fields are required (e.g. setting different tokens)

    /// Sign the transaction and produce a UserCommand with the signature fields filled
    pub fn sign_and_build(self, signer: PublicKey) -> UserCommand {
        UserCommand::SignedCommand(SignedCommand {
            payload: SignedCommandPayload {
                common: self.common,
                body: SignedCommandPayloadBody::PaymentPayload(self.payment),
            },
            signer,
            signature: Default::default(), // TODO: Add signing logic once this is available
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_crypto::prelude::Base58Encodable;

    #[test]
    fn can_build_default() {
        let cmd = SignedTransferCommandBuilder::new().sign_and_build(PublicKey::default());
        assert_eq!(cmd, UserCommand::default())
    }

    #[test]
    fn can_set_receiver_pk() {
        let to = PublicKey::from_base58("B62qonDZEKYULNkfq7WGu1Z881YBRnMSuBGGX5DhnTv26mUyvN99mpo")
            .unwrap();
        let cmd = SignedTransferCommandBuilder::new()
            .to(to.clone())
            .sign_and_build(PublicKey::default());

        match cmd {
            UserCommand::SignedCommand(SignedCommand {
                payload:
                    SignedCommandPayload {
                        body:
                            SignedCommandPayloadBody::PaymentPayload(PaymentPayload {
                                receiver_pk, ..
                            }),
                        ..
                    },
                ..
            }) => {
                assert_eq!(receiver_pk, to);
            }
        }
    }

    #[test]
    fn can_set_source_pk() {
        let from =
            PublicKey::from_base58("B62qonDZEKYULNkfq7WGu1Z881YBRnMSuBGGX5DhnTv26mUyvN99mpo")
                .unwrap();
        let cmd = SignedTransferCommandBuilder::new()
            .from(from.clone())
            .sign_and_build(PublicKey::default());

        match cmd {
            UserCommand::SignedCommand(SignedCommand {
                payload:
                    SignedCommandPayload {
                        body:
                            SignedCommandPayloadBody::PaymentPayload(PaymentPayload {
                                source_pk, ..
                            }),
                        ..
                    },
                ..
            }) => {
                assert_eq!(source_pk, from);
            }
        }
    }

    #[test]
    fn can_set_amount() {
        let set_amount = Amount(99);
        let cmd = SignedTransferCommandBuilder::new()
            .amount(set_amount)
            .sign_and_build(PublicKey::default());

        match cmd {
            UserCommand::SignedCommand(SignedCommand {
                payload:
                    SignedCommandPayload {
                        body:
                            SignedCommandPayloadBody::PaymentPayload(PaymentPayload { amount, .. }),
                        ..
                    },
                ..
            }) => {
                assert_eq!(set_amount, amount);
            }
        }
    }

    #[test]
    fn can_set_fee() {
        let set_fee = Amount(99);
        let cmd = SignedTransferCommandBuilder::new()
            .fee(set_fee)
            .sign_and_build(PublicKey::default());

        match cmd {
            UserCommand::SignedCommand(SignedCommand {
                payload:
                    SignedCommandPayload {
                        common: SignedCommandPayloadCommon { fee, .. },
                        ..
                    },
                ..
            }) => {
                assert_eq!(set_fee, fee);
            }
        }
    }

    #[test]
    fn can_set_nonce() {
        let set_nonce = ExtendedU32(4);
        let cmd = SignedTransferCommandBuilder::new()
            .nonce(set_nonce)
            .sign_and_build(PublicKey::default());

        match cmd {
            UserCommand::SignedCommand(SignedCommand {
                payload:
                    SignedCommandPayload {
                        common: SignedCommandPayloadCommon { nonce, .. },
                        ..
                    },
                ..
            }) => {
                assert_eq!(set_nonce, nonce);
            }
        }
    }
}
