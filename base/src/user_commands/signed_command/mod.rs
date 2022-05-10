// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Signed commands are commands that require signing with some accounts private key

pub mod builder;

use crate::numbers::{AccountNonce, Amount, GlobalSlotNumber, TokenId};
use crate::user_commands::memo::SignedCommandMemo;
use crate::user_commands::payment::PaymentPayload;
use crate::verifiable::Verifiable;

use proof_systems::mina_hasher::{Hashable, ROInput};
use proof_systems::mina_signer::{CompressedPubKey, Keypair, NetworkId, Signature, Signer};

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

impl SignedCommand {
    /// Sign a SignedCommandPayload to construct a SignedCommand
    pub fn from_payload(
        payload: SignedCommandPayload,
        keypair: Keypair,
        network: NetworkId,
    ) -> Self {
        // This should change to create_kimchi after fork
        let mut ctx = proof_systems::mina_signer::create_legacy::<SignedCommandPayload>(network);
        let signature = ctx.sign(&keypair, &payload);

        SignedCommand {
            payload,
            signer: keypair.public.into_compressed(),
            signature,
        }
    }
}

use proof_systems::mina_signer;
use proof_systems::mina_signer::PubKey;

impl Verifiable for SignedCommand {
    type Sup = ();

    fn verify(&self, _data: Self::Sup) -> bool {
        let mut mainnet_ctx = mina_signer::create_legacy(NetworkId::MAINNET);
        // do a slightly sketchy conversion via address. Safe to unwrap as we know it was valid to begin with
        let signer_uncompressed = PubKey::from_address(&self.signer.into_address()).unwrap();

        mainnet_ctx.verify(&self.signature, &signer_uncompressed, &self.payload)
    }
}

/// The part of a signed command that needs to be serialized and signed
#[derive(Clone, PartialEq, Debug)]
pub struct SignedCommandPayload {
    /// Fields common to all command types
    pub common: SignedCommandPayloadCommon,
    /// Fields that depend on the type of command (e.g. payment, snapp, etc)
    pub body: SignedCommandPayloadBody,
}

impl SignedCommandPayload {
    /// Convert into a signed command by signing with the given keypair and network ID
    pub fn into_signed_command(self, keypair: Keypair, network: NetworkId) -> SignedCommand {
        SignedCommand::from_payload(self, keypair, network)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_commands::SignedCommand;
    use proof_systems::mina_signer;
    use proof_systems::mina_signer::{CompressedPubKey, Keypair, NetworkId, PubKey, Signer};

    // Credit to the proof-systems repo tests from where this macro was taken
    macro_rules! assert_sign_verify_tx {
        ($tx_type:expr, $sec_key:expr, $source_address:expr, $receiver_address:expr, $amount:expr, $fee:expr,
         $nonce:expr, $valid_until:expr, $memo:expr, $testnet_target:expr, $mainnet_target:expr) => {
            let kp = Keypair::from_hex($sec_key).expect("failed to create keypair");
            assert_eq!(
                kp.public,
                PubKey::from_address($source_address).expect("invalid source address")
            );

            let builder = builder::SignedTransferCommandBuilder::new(
                CompressedPubKey::from_address($source_address).expect("invalid source address"),
                CompressedPubKey::from_address($receiver_address)
                    .expect("invalid receiver address"),
                $amount,
                $fee,
                $nonce,
            )
            .valid_until($valid_until)
            .memo(SignedCommandMemo::try_from($memo).expect("invalid memo string"));

            let mut payload = builder.build();

            let testnet_sig =
                SignedCommand::from_payload(payload.clone(), kp, NetworkId::TESTNET).signature;
            let mainnet_sig =
                SignedCommand::from_payload(payload.clone(), kp, NetworkId::MAINNET).signature;

            // Context for verification
            let mut testnet_ctx = mina_signer::create_legacy(NetworkId::TESTNET);
            let mut mainnet_ctx = mina_signer::create_legacy(NetworkId::MAINNET);

            // Signing checks
            assert_ne!(testnet_sig, mainnet_sig); // Testnet and mainnet sigs are not equal
            assert_eq!(testnet_sig.to_string(), $testnet_target); // Testnet target check
            assert_eq!(mainnet_sig.to_string(), $mainnet_target); // Mainnet target check

            // Verification checks
            assert_eq!(testnet_ctx.verify(&testnet_sig, &kp.public, &payload), true);
            assert_eq!(mainnet_ctx.verify(&mainnet_sig, &kp.public, &payload), true);

            // Fails verification on the other network
            assert_eq!(
                mainnet_ctx.verify(&testnet_sig, &kp.public, &payload),
                false
            );
            assert_eq!(
                testnet_ctx.verify(&mainnet_sig, &kp.public, &payload),
                false
            );

            // Flip some bits, its should no longer pass verification
            payload.common.valid_until.0 = !payload.common.valid_until.0;
            assert_eq!(
                mainnet_ctx.verify(&testnet_sig, &kp.public, &payload),
                false
            );
            assert_eq!(
                testnet_ctx.verify(&mainnet_sig, &kp.public, &payload),
                false
            );
        };
    }

    #[test]
    fn sign_payment_test_1() {
        assert_sign_verify_tx!(
            /* Transaction type   */ TransactionType::PaymentTx,
            /* sender secret key  */ "164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718",
            /* source address     */ "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            /* receiver address   */ "B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt",
            /* amount             */ 1729000000000_u64,
            /* fee                */ 2000000000_u32,
            /* nonce              */ 16_u32,
            /* valid until        */ 271828_u32,
            /* memo               */ "Hello Mina!",
            /* testntet signature */ "11a36a8dfe5b857b95a2a7b7b17c62c3ea33411ae6f4eb3a907064aecae353c60794f1d0288322fe3f8bb69d6fabd4fd7c15f8d09f8783b2f087a80407e299af",
            /* mainnet signature  */ "124c592178ed380cdffb11a9f8e1521bf940e39c13f37ba4c55bb4454ea69fba3c3595a55b06dac86261bb8ab97126bf3f7fff70270300cb97ff41401a5ef789"
        );
    }

    #[test]
    fn sign_payment_test_2() {
        assert_sign_verify_tx!(
            /* Transaction type  */ TransactionType::PaymentTx,
            /* sender secret key */ "3414fc16e86e6ac272fda03cf8dcb4d7d47af91b4b726494dab43bf773ce1779",
            /* source address    */ "B62qoG5Yk4iVxpyczUrBNpwtx2xunhL48dydN53A2VjoRwF8NUTbVr4",
            /* receiver address  */ "B62qrKG4Z8hnzZqp1AL8WsQhQYah3quN1qUj3SyfJA8Lw135qWWg1mi",
            /* amount            */ 314159265359_u64,
            /* fee               */ 1618033988_u32,
            /* nonce             */ 0_u32,
            /* valid until       */ 4294967295_u32,
            /* memo              */ "",
            /* testnet signature */ "23a9e2375dd3d0cd061e05c33361e0ba270bf689c4945262abdcc81d7083d8c311ae46b8bebfc98c584e2fb54566851919b58cf0917a256d2c1113daa1ccb27f",
            /* mainnet signature */ "204eb1a37e56d0255921edd5a7903c210730b289a622d45ed63a52d9e3e461d13dfcf301da98e218563893e6b30fa327600c5ff0788108652a06b970823a4124"
        );
    }

    #[test]
    fn sign_payment_test_3() {
        assert_sign_verify_tx!(
            /* Transaction type  */ TransactionType::PaymentTx,
            /* sender secret key */ "3414fc16e86e6ac272fda03cf8dcb4d7d47af91b4b726494dab43bf773ce1779",
            /* source address    */ "B62qoG5Yk4iVxpyczUrBNpwtx2xunhL48dydN53A2VjoRwF8NUTbVr4",
            /* receiver address  */ "B62qoqiAgERjCjXhofXiD7cMLJSKD8hE8ZtMh4jX5MPNgKB4CFxxm1N",
            /* amount            */ 271828182845904_u64,
            /* fee               */ 100000_u32,
            /* nonce             */ 5687_u32,
            /* valid until       */ 4294967295_u32,
            /* memo              */ "01234567890123456789012345678901",
            /* testnet signature */ "2b4d0bffcb57981d11a93c05b17672b7be700d42af8496e1ba344394da5d0b0b0432c1e8a77ee1bd4b8ef6449297f7ed4956b81df95bdc6ac95d128984f77205",
            /* mainnet signature */ "076d8ebca8ccbfd9c8297a768f756ff9d08c049e585c12c636d57ffcee7f6b3b1bd4b9bd42cc2cbee34b329adbfc5127fe5a2ceea45b7f55a1048b7f1a9f7559"
        );
    }

    #[test]
    fn sign_payment_test_4() {
        assert_sign_verify_tx!(
            /* Transaction type  */ TransactionType::PaymentTx,
            /* sender secret key */ "1dee867358d4000f1dafa5978341fb515f89eeddbe450bd57df091f1e63d4444",
            /* source address    */ "B62qoqiAgERjCjXhofXiD7cMLJSKD8hE8ZtMh4jX5MPNgKB4CFxxm1N",
            /* receiver address  */ "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            /* amount            */ 0_u32,
            /* fee               */ 2000000000_u32,
            /* nonce             */ 0_u32,
            /* valid until       */ 1982_u32,
            /* memo              */ "",
            /* testnet signature */ "25bb730a25ce7180b1e5766ff8cc67452631ee46e2d255bccab8662e5f1f0c850a4bb90b3e7399e935fff7f1a06195c6ef89891c0260331b9f381a13e5507a4c",
            /* mainnet signature */ "058ed7fb4e17d9d400acca06fe20ca8efca2af4ac9a3ed279911b0bf93c45eea0e8961519b703c2fd0e431061d8997cac4a7574e622c0675227d27ce2ff357d9"
        );
    }
}
