// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_consensus::genesis::*;
    use mina_crypto::prelude::*;
    use mina_rs_base::types::*;
    use time::macros::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_genesis_mainnet_wasm() {
        test_genesis_mainnet().unwrap()
    }

    // https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#611-genesis-block
    #[test]
    fn test_genesis_mainnet() -> anyhow::Result<()> {
        let et = ExternalTransition::from_genesis_config(&MAINNET_CONFIG);

        let bs = &et.protocol_state.body.blockchain_state;
        assert_eq!(bs.timestamp.datetime(), datetime!(2021-03-17 00:00:0 UTC));
        assert_eq!(bs.snarked_next_available_token.0, 2);
        assert_eq!(
            bs.snarked_ledger_hash.to_base58_string()?,
            "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
        );
        assert_eq!(
            bs.genesis_ledger_hash.to_base58_string()?,
            "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
        );
        assert_eq!(
            bs.staged_ledger_hash
                .pending_coinbase_hash
                .to_base58_string()?,
            "2n1tLdP2gkifmyVmrmzYXTS4ohPbZPJn6Qq4x55ywrbRWB4543cC"
        );
        assert_eq!(
            bs.staged_ledger_hash
                .non_snark
                .ledger_hash
                .to_base58_string()?,
            "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
        );

        let cs = &et.protocol_state.body.consensus_state;
        assert_eq!(cs.blockchain_length, 1_u32.into());
        assert_eq!(cs.epoch_count, 0_u32.into());
        assert_eq!(cs.min_window_density, 77_u32.into());
        assert_eq!(cs.sub_window_densities.len(), 11);
        assert_eq!(
            cs.sub_window_densities(),
            vec![1, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]
        );
        assert_eq!(
            cs.last_vrf_output.0,
            base64::decode("NfThG1r1GxQuhaGLSJWGxcpv24SudtXG4etB0TnGqwg=")?
        );
        assert_eq!(cs.total_currency.to_string(), "805385692.840039233");
        assert_eq!(cs.curr_global_slot.slot_number, 0_u32.into());
        assert_eq!(cs.curr_global_slot.slots_per_epoch, 7140_u32.into());
        assert_eq!(cs.global_slot_since_genesis, 0_u32.into());

        {
            let sed = &cs.staking_epoch_data;
            assert_eq!(
                sed.ledger.hash.to_base58_string()?,
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
            );
            assert_eq!(sed.ledger.total_currency.0, 805385692840039233);
            assert_eq!(
                sed.seed.to_base58_string()?,
                "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA"
            );
            assert_eq!(
                sed.start_checkpoint.to_base58_string(),
                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
            );
            assert_eq!(
                sed.lock_checkpoint.to_base58_string(),
                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
            );
            assert_eq!(sed.epoch_length, 1_u32.into());
        }
        {
            let ned = &cs.next_epoch_data;
            assert_eq!(
                ned.ledger.hash.to_base58_string()?,
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
            );
            assert_eq!(ned.ledger.total_currency.0, 805385692840039233);
            assert_eq!(
                ned.seed.to_base58_string()?,
                "2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt"
            );
            assert_eq!(
                ned.start_checkpoint.to_base58_string(),
                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
            );
            assert_eq!(
                ned.lock_checkpoint.to_base58_string(),
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d"
            );
            assert_eq!(ned.epoch_length, 2_u32.into());
        }

        assert_eq!(cs.has_ancestor_in_same_checkpoint_window, true);
        assert_eq!(
            cs.block_stake_winner.into_address(),
            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg"
        );
        assert_eq!(
            cs.block_creator.into_address(),
            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg"
        );
        assert_eq!(
            cs.coinbase_receiver.into_address(),
            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg"
        );
        assert_eq!(cs.supercharge_coinbase, true);
        assert_eq!(
            et.protocol_state.previous_state_hash.to_base58_string(),
            "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d"
        );
        assert_eq!(
            et.protocol_state.body.genesis_state_hash.to_base58_string(),
            "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d"
        );

        Ok(())
    }
}
