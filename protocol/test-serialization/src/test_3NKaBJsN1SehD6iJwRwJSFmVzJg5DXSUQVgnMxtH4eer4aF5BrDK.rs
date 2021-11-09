// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use chrono::prelude::*;
    use mina_rs_base::staged_ledger_diff::{
        SignedCommandMemo, SignedCommandPayloadBody, UserCommand,
    };
    use pretty_assertions::assert_eq;
    use test_fixtures::*;

    // https://minaexplorer.com/block/3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK
    #[test]
    fn test_block() -> anyhow::Result<()> {
        let et = TEST_BLOCKS
            .get("3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex")
            .unwrap()
            .external_transition()?;

        let body = &et.protocol_state.body;

        let protocol_state = &body.blockchain_state;
        assert_eq!(protocol_state.timestamp.epoch_millis(), 1636092900000);
        assert_eq!(
            protocol_state.timestamp.datetime(),
            Utc.ymd(2021, 11, 5).and_hms_nano(6, 15, 0, 0)
        );

        let consensus_state = &body.consensus_state;
        assert_eq!(*consensus_state.blockchain_length, 77748);
        assert_eq!(*consensus_state.epoch_count, 15);
        assert_eq!(*consensus_state.curr_global_slot.slot_number, 111965);
        let bytes = bs58::decode("B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN")
            .into_vec()
            .unwrap();
        // TODO: Validate full bytes vec with salted mainnet signature
        assert_eq!(consensus_state.block_creator.poly.x[..], bytes[3..35]);

        assert_eq!(
            consensus_state.total_currency.to_formatted_string(),
            "867667132.840039233"
        );

        assert!(&et.staged_ledger_diff.diff.diff_one().is_none());
        let commands = &et.staged_ledger_diff.diff.diff_two().commands;
        assert_eq!(commands.len(), 3);

        match &commands[0].data {
            UserCommand::SignedCommand(command) => {
                assert_eq!(command.payload.common.nonce.0, 5694);
                assert_eq!(
                    command.payload.common.memo.0,
                    SignedCommandMemo::try_from("FPayment").unwrap().0,
                );
                assert_eq!(
                    command.payload.common.fee.to_formatted_string(),
                    "0.010000000"
                );
                match &command.payload.body {
                    SignedCommandPayloadBody::PaymentPayload(body) => {
                        assert_eq!(body.amount.to_formatted_string(), "0.027370000");
                        let bytes =
                            bs58::decode("B62qoSuxNqwogusxxZbs3gpJUxCCN4GZEv21FX8S2DtNpToLgKnrexM")
                                .into_vec()
                                .unwrap();
                        // TODO: Validate full bytes vec with salted mainnet signature
                        assert_eq!(body.source_pk.x[..], bytes[3..35]);
                        let bytes =
                            bs58::decode("B62qn2MtuQ9GyyVnotUHB9Ehp9EZre5m6TYpGx64tBCDHHBZFZRURnL")
                                .into_vec()
                                .unwrap();
                        // TODO: Validate full bytes vec with salted mainnet signature
                        assert_eq!(body.receiver_pk.x[..], bytes[3..35]);
                    }
                    _ => bail!(
                        "PaymentPayload expected, but found: {:#?}",
                        command.payload.body
                    ),
                };
            }
            _ => bail!("SignedCommand expected, but found: {:#?}", commands[0].data),
        }

        Ok(())
    }
}
