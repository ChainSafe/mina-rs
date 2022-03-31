// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
use anyhow::bail;
    use mina_crypto::hash::*;
    use mina_crypto::prelude::*;
    use mina_crypto::signature::{Signature};
    use mina_rs_base::types::*;
    use mina_serialization_types::v1::ExternalTransitionV1;
    use pretty_assertions::assert_eq;
    use test_fixtures::*;
    use time::macros::*;
    use wasm_bindgen_test::*;

    use mina_signer::CompressedPubKey;
    use o1_utils::field_helpers::FieldHelpers;

    #[wasm_bindgen_test]
    fn test_block_wasm() {
        test_block().unwrap()
    }

    // https://minaexplorer.com/block/3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK
    // https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    #[test]
    fn test_block() -> anyhow::Result<()> {
        let et: ExternalTransitionV1 = TEST_BLOCKS
            .get("3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex")
            .unwrap()
            .external_transitionv1()?;

        let protocol_state = &et.0.t.protocol_state;
        assert_eq!(
            StateHash::from(protocol_state.t.t.previous_state_hash.clone()).to_base58_string(),
            "3NKDdX6eVtAgmmTVxaFLnnPPrsGKgVepG2k5cf8HocgSw6ps8Sww"
        );

        let body = &et.0.t.protocol_state.t.t.body;
        assert_eq!(
            StateHash::from(body.t.t.genesis_state_hash.clone()).to_base58_string(),
            "3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ"
        );
        let blockchain_state = &body.t.t.blockchain_state;
        let non_snark = &blockchain_state.t.t.staged_ledger_hash.t.t.non_snark;
        assert_eq!(
            LedgerHash::from(non_snark.t.ledger_hash.clone()).to_base58_string(),
            "jwD5Kx1GtLKJGSWufhkvCn8m7EFLm2LmAM7neyzLtTiN8wyn2po"
        );

        let bytes = bs58::decode("UworXDykADr3Lte856ePMsdawpTVhKLKT9Y3UKha7Tpbt4V1JP")
            .into_vec()
            .unwrap();
        assert_eq!(non_snark.t.aux_hash.t[..], bytes[1..33]);
        assert_eq!(
            AuxHash::from(non_snark.t.aux_hash.t.clone()).to_base58_string(),
            "UworXDykADr3Lte856ePMsdawpTVhKLKT9Y3UKha7Tpbt4V1JP"
        );

        let bytes = bs58::decode("XbwfEKZjgcZiyDhHRZjHUx72TuxpnuzLPwVYpVWkMAAXkSy7go")
            .into_vec()
            .unwrap();
        assert_eq!(non_snark.t.pending_coinbase_aux.t[..], bytes[1..33]);
        assert_eq!(
            PendingCoinbaseAuxHash(non_snark.t.pending_coinbase_aux.t.clone()).to_base58_string(),
            "XbwfEKZjgcZiyDhHRZjHUx72TuxpnuzLPwVYpVWkMAAXkSy7go"
        );

        assert_eq!(
            CoinBaseHash::from(
                blockchain_state
                    .t
                    .t
                    .staged_ledger_hash
                    .t
                    .t
                    .pending_coinbase_hash
                    .t
                    .clone()
            )
            .to_base58_string(),
            "2mzpdUi5ddLicLGUns4iYFiNahL5B5cPkTUot83v2moNtr4mzRYf"
        );
        assert_eq!(
            SnarkedLedgerHash::from(blockchain_state.t.t.snarked_ledger_hash.clone())
                .to_base58_string(),
            "jxkQm8ge9sYPwPyUYUMZ6wr7SQ6Pit5szbRvPmEzYKQQZAnACyC"
        );
        assert_eq!(
            SnarkedLedgerHash::from(blockchain_state.t.t.genesis_ledger_hash.clone())
                .to_base58_string(),
            "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
        );
        assert_eq!(blockchain_state.t.t.snarked_next_available_token.t.t.t, 2);
        assert_eq!(
            BlockTime::from(blockchain_state.t.t.timestamp.clone()).epoch_millis(),
            1636092900000
        );
        assert_eq!(
            BlockTime::from(blockchain_state.t.t.timestamp.clone()).datetime(),
            datetime!(2021-11-05 06:15:00 UTC)
        );

        let consensus_state = &body.t.t.consensus_state;
        assert_eq!(consensus_state.t.t.blockchain_length.t.t, 77748);
        assert_eq!(consensus_state.t.t.epoch_count.t.t, 15);
        assert_eq!(consensus_state.t.t.min_window_density.t.t, 33);
        assert_eq!(
            ConsensusState::from(consensus_state.clone()).sub_window_densities(),
            vec![6, 1, 3, 5, 4, 3, 5, 7, 4, 5, 6,]
        );

        let bytes = base64::decode_config(
            "WNAmmaRL7XzyhZHiz276MbnBv4YUIJRGf9P_Xu0RBAA=",
            base64::URL_SAFE,
        )
        .unwrap();
        assert_eq!(
            VrfOutputTruncated::from(consensus_state.t.t.last_vrf_output.t.clone()).as_ref(),
            &bytes[..]
        );

        assert_eq!(
            Amount::from(consensus_state.t.t.total_currency.t.t).to_string(),
            "867667132.840039233"
        );
        assert_eq!(
            consensus_state.t.t.curr_global_slot.t.t.slot_number.t.t,
            111965
        );
        assert_eq!(
            consensus_state.t.t.curr_global_slot.t.t.slots_per_epoch.t.t,
            7140
        );
        assert_eq!(consensus_state.t.t.global_slot_since_genesis.t.t, 111965);

        let staking_epoch_data = &consensus_state.t.t.staking_epoch_data;
        assert_eq!(
            LedgerHash::from(staking_epoch_data.t.t.ledger.t.t.hash.clone()).to_base58_string(),
            "jxn15ATGoe4WGgYpbssxJH9XW8NXRDy22WvSsBqvMqcnLPgPAwN"
        );
        assert_eq!(
            Amount::from(staking_epoch_data.t.t.ledger.t.t.total_currency.t.t).to_string(),
            "861208012.840039233"
        );
        assert_eq!(
            EpochSeed::from(staking_epoch_data.t.t.seed.clone()).to_base58_string(),
            "2vao4i3odTHZVRbEhdkKvLoD1rW2UuiVaayVFosYtkghABg29o7i"
        );
        assert_eq!(
            StateHash::from(staking_epoch_data.t.t.start_checkpoint.clone()).to_base58_string(),
            "3NLM6x7j2Z68e8gGspyvc1aU884uU6yWkwz9aW127BFckn9b5uvo"
        );
        assert_eq!(
            StateHash::from(staking_epoch_data.t.t.lock_checkpoint.clone()).to_base58_string(),
            "3NLiFhztdCsuWSociNGMspidiYkyqNKZw6ufH7jqbgQtEgGtBb2P"
        );
        assert_eq!(staking_epoch_data.t.t.epoch_length.t.t, 4697);

        let next_epoch_data = &consensus_state.t.t.next_epoch_data;
        assert_eq!(
            LedgerHash::from(next_epoch_data.t.t.ledger.t.t.hash.clone()).to_base58_string(),
            "jwAXd4GZgxE3YCwqs99g4MpLNiEV2ZfZPstyah4jxo753AVgL6R"
        );
        assert_eq!(
            Amount::from(next_epoch_data.t.t.ledger.t.t.total_currency.t.t).to_string(),
            "864998092.840039233"
        );
        assert_eq!(
            EpochSeed::from(next_epoch_data.t.t.seed.clone()).to_base58_string(),
            "2vbUkQGF5swXK7PNaAJDUQirW1fbZiUJDzbBKwfPGdJXZiryburD"
        );
        assert_eq!(
            StateHash::from(next_epoch_data.t.t.start_checkpoint.clone()).to_base58_string(),
            "3NLkdXKqoHfwZ5jT1uxSY3eoFy3C2jpAUFZ1Y6eSMsE66MNJqErx"
        );
        assert_eq!(
            StateHash::from(next_epoch_data.t.t.lock_checkpoint.clone()).to_base58_string(),
            "3NLW5kBi9nXDzzdr2C3p9X6QaKaASMaVHp3otwreKXKJToUNK7yu"
        );
        assert_eq!(next_epoch_data.t.t.epoch_length.t.t, 3285);

        assert_eq!(
            consensus_state.t.t.has_ancestor_in_same_checkpoint_window,
            true
        );
        assert_eq!(
            CompressedPubKey::from(consensus_state.t.t.block_stake_winner.clone().into()).into_address(),
            "B62qmsYXFNNE565yv7bEMPsPnpRCsMErf7J2v5jMnuKQ1jgwZS8BzXS"
        );
        assert_eq!(
            CompressedPubKey::from(consensus_state.t.t.block_creator.clone().into()).into_address(),
            "B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN"
        );
        assert_eq!(
            CompressedPubKey::from(consensus_state.t.t.coinbase_receiver.clone().into()).into_address(),
            "B62qk9WYHu2PBYv4EyEubnVQURcwpiV2ysuYYoMdwi8YTnwZQ7H4bLM"
        );
        assert_eq!(consensus_state.t.t.supercharge_coinbase, false);

        let bytes = bs58::decode("B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN")
            .into_vec()
            .unwrap();
        // TODO: Validate full bytes vec with salted mainnet signature
        assert_eq!(consensus_state.t.t.block_creator.0.t.t.x[..], bytes[3..35]);

        assert_eq!(
            Amount::from(consensus_state.t.t.total_currency.t.t).to_string(),
            "867667132.840039233"
        );

        let constants = &body.t.t.constants;
        assert_eq!(constants.t.t.k.t.t, 290);
        assert_eq!(constants.t.t.slots_per_epoch.t.t, 7140);
        assert_eq!(constants.t.t.slots_per_sub_window.t.t, 7);
        assert_eq!(constants.t.t.delta.t.t, 0);
        assert_eq!(
            BlockTime::from(constants.t.t.genesis_state_timestamp.clone()).epoch_millis(),
            1615939200000
        );

        /*
        TODO: Verify protocol_state_proof
        AQEBAQEBAQEBAQABAfyCZ8XD4he3QAH80HIgGfAezhgAAQH8k9ybHSm_cswB_JQesWKV8mPiAAEB_CFDKDlOgTyvAfxQGTylbUUpgAABAAEB_IlOihwF_6cPAfy8HOdGfm3muwABACNq3aIW99rvmPThUJOt7sTCDHKGhd14jtt6WT6kuFYzAQCn9Tpe5H8iu4eCAUs2O4jrZQzs6DrhpxGQc6AzAkteOQEAAQH8xQZ4fhDIOj4B_F6Fr8O5CTsMAAEBAQEAAQH8Qm91cFzlxuYB_Cz4ZjYnTwVyAAEBAAEB_OZAKPtMoib5AfzQLLCTKmHongABAQABAfyG02v2I-l6jAH8iEeTiKBbKjkAAQEAAQH84lhBVf2xpzEB_BHUmoZUZqv_AAEBAAEB_OFYodLGFJH-AfzMLIm8ErnpVAABAQABAfxMGNX65P-rNgH8MeXQ9yLbruwAAQEAAQH8-fFQ7ZTD2TwB_EPopZeAO7DyAAEBAAEB_AVFodFdv3riAfxCkBzGujjAjgABAQABAfyFmgu3zzh_iwH8a-3QksBFIDoAAQEAAQH8EyV9kj2hjxgB_Cjq1rVlPnA2AAEBAAEB_DFjvfxSXfMNAfwQ03vM-qWnKgABAQABAfw9u8uYpRjVGwH8auCvoVqVrO4AAQEAAQH8_4zOksnfvWkB_H0-zs3vUPLwAAEBAAEB_Np8pO8v4SKuAfw9_QTgxhgP6QABAQABAfxOPTWunUVTRAH8IJwLvYO0jRoAAQEAAQH8NWq1qzPRU3IB_M1l0lZM_zlnAAEBAAEB_J2IofKS20MhAfyT6M0RL_FNXwABAQABAfzc3NisCDJ9cAH8mHqRgVdvGJ8AAAEAAQEB_Oc4nCuNu3lxAfyS4L6rm1VgHAH8Fi1AkXr5xRwB_H-LBb1N9VI7AAGVHNYP3HXWMiP5EYHwdZSahxjzCaid3EcbBzc1l2mSIFiwoLM6mA3b-sLAjDM00_Mv1kn8JdKSBz7XFUBdx9wFAQEBAQEBAAEB_H_cKwozftw5AfxzlUcCknjPGgABAQABAfyHYkiXOVf-rAH8wIdrKUAWuH4AAQEAAQH8yiZH-pNlO5QB_EkFjtbxE2TbAAEBAAEB_Ob-bwZ88i3EAfyyVmk5Zf4CbAABAQABAfwz3DYG5_0-ygH8FxZPvQICA64AAQEAAQH8cbO-YveQxEEB_E41RZxRqpJlAAEBAAEB_Oo1cWmj6IExAfwkIdFslaKGqAABAQABAfxKgrUH7qwW9QH8uHoxdQDJBAMAAQEAAQH8LW1TBwUeAfgB_BOTZGJKRlWrAAEBAAEB_OJQNmjd_rBbAfwX_tnoJBlnZAABAQABAfzR__6qjUF5QQH82cvkkjXbCtsAAQEAAQH86Fufzsl-WBsB_Niuie-SE0TFAAEBAAEB_ARddbjD5m0TAfyrzYXQSzCS-QABAQABAfyUxo3LqdBbmAH8lwAIZXaO4B8AAQEAAQH8ERPxGwUJEQAB_AYX7pGEDNoxAAEBAAEB_Gn4iXcgSl-uAfzthAXwXkQ10QABAQABAfzPbCxIxDk6WgH8EdD4I5p-FrIAAAEBAQEBAAEB_PRiou0KbJASAfwzYzmUKFHRKgABAQABAfz3c_qAAg-WyQH83ZszarSNKcsAAQEAAQH8HsHUvGwApOUB_Eqiqp5SyuCHAAEBAAEB_P4hxMctvUjRAfxnHuBu_HXT4QABAQABAfymfTactcwx1gH8Icp3IyaujmAAAQEAAQH8cDGngppKGzoB_MylECYCZqXGAAEBAAEB_Cxrkn7squFFAfzvzIGxZqssMwABAQABAfx4bvwgL0G5wwH8HuPWD2rzWHgAAQEAAQH8J-gjjF9EPfYB_LaiHNuAl65mAAEBAAEB_B5av2NUMr-2AfzMwUabw05SRQABAQABAfxILOUvISw1mgH8_lcHUNvTXp0AAQEAAQH8ASWoSqRxiCQB_MC-AL-kH_i7AAEBAAEB_LESBxwvL7WUAfy5GKn8G6fgOAABAQABAfwgtHDI3HuiSwH8DSB2EBWXgvwAAQEAAQH8J51m68nA0wkB_PcjXo2cmFjiAAEBAAEB_GhNOayv4cvqAfy76E0UHLopBQABAQABAfzgUeAo4PO5ugH8VpiUeeuLT1YAAAABAAECSDA5pdNBwJPjnxV4iNvGpdfmkzXqi8Uc11QcnW_7DBp8lKx_rowAATmxVG6S4JppbpZbJU3aZZXPa52AkyJmOLFkcQJiRYmrPagLNz8DTCQsdcQnFXLM-2f8qO2q3iMFgWeOOsZ4WXrHUsaCLimDl5zUjJo9vLv14lmN-lZ08gYBAgEBAQEAAQH8_p0afZ5vlFsB_J5qFOanysI0AAEBAAEB_AdAbP5WNI7FAfxSJQ6tLG4rcwABAQABAfyVnS4O4rVdaQH8xQbAV7LRwqcAAQEAAQH8osKaiVy12owB_O5aOFBJp21xAAEBAAEB_AaY9zrzAK7lAfwYyRPFhYzZVAABAQABAfxgblAzm_WBbQH8dOLDRt4RoM0AAQEAAQH8hyrBb9OhscUB_K2obrL_-efFAAEBAAEB_DjNh4bpMmpoAfxu_IqqVSc9FwABAQABAfyjLEwAcNxsFAH82DQn99QzR00AAQEAAQH8Yr3wmuuScUsB_AZpD-r3hK06AAEBAAEB_KOcxwA41xDdAfyx4TP4C6FIJwABAQABAfyfNjNgByGJAgH8GH8XNjCbmxsAAQEAAQH8jv8noOn7nvMB_CKCmA5iH59oAAEBAAEB_DpP60G3dOMXAfxXNTEcN1JMggABAQABAfxAM3V-VupUlwH8kc83OahzhmIAAQEAAQH80Ck9Sa9pkfIB_AslFCAAktlfAAEBAAEB_Kvb_sxftPaSAfxhccxlfGMskAABAQABAfz85W87LwKKvwH8G6-4q4yIrK0AAAEBAQEAAQH8DMVpym0zoQgB_IuGEn36D_DDAAEBAAEB_IkAs_6a1ot7AfwRKLlqjdLzswABAQABAfzBBzWGcLjPcwH8nOfrwyXsm3IAAQEAAQH8JU-rVyi2WwoB_PKA6zqDmK-xAAEBAAEB_Lkqp1a0cHOtAfz8nvHVI_lPNgABAQABAfwAfC-OYhyHWQH8h8wmonP2x5wAAQEAAQH8r_K2nh2CVCMB_H71ffbRa7nVAAEBAAEB_PaGkKDQ93sUAfxoKiRAzmJeYgABAQABAfwOrVYyYxvGrwH8--EfoRBygAkAAQEAAQH8kUGsyr4eWPkB_KbJtz6Z1R5XAAEBAAEB_L3DZM2jUE6qAfxoxf7BCucU2AABAQABAfxt3l6C36wdsgH8pQfbxReiCP4AAQEAAQH8f6rm6dYPToIB_Cx_uU6YOvb8AAEBAAEB_MoEG3EriDHDAfwpJq62x6w5kQABAQABAfzvUYH9R48P3AH8h5U7xEN6qQAAAQEAAQH8vzKG0R7YOGAB_KsFqqJwvLP5AAEBAAEB_FpHr-Xg0nWUAfz20sOuAqfL0QABAQABAfwEfC359g94vgH8VOL7MpFYPeEAAAEBAQHFn4aQFgBI2GDXcPdOc1EsQZ8KS4vGw9hYCk2yDF5eOgEBbVGq6-0G0nSa8ok7FYnvnJ0ObTJ9hN1K1DzP_I-pzTMBATzZ-9xtoegAWYXcFBvq2o0RwFpAXF_bYcNv_MBJDfcmAQF7f6cCIRNEB9wpHd3XiN9QuLeKiuPquoBTLaJT_ECpCQEFvCv5PCUdi9A-RQnu66Y8oaW6qAxQ3dSPBK4Ox3c3EQN0zzBbePrf-8hTsYmRUAEeHxFi6X_NzApRJWLEZjh3IhDCvj6Bp2c2V1PavlSytfPc9Wwgeyxz-eIngUJt16wTVSH3tu47eimSxtXKqyXtgbSqOzxFEYmJCUHHJi0c9y-nEpaXbUtL7BCGwlaB-EXsQqTXAbIHsfKHhPAsuCEmCwEBlq4Pa9LiSB9sBxPIpfnjs5edIjzY2_Uju71rcd74OyQBAW-qp5OUEo9NggJljqLtU5ceSwaAo6jWY0FYiRkxRWMsAQH_v28Lo818reRiIpPUzqASYqKfNzC9iCO9e0HrHsuaBgEBARbL0NaRWMlSRSNMwZSy2vAqL3hHkv8YHzIzfXSUDpcWAQHSsSQdmUVrzE7AUA9zE8FeBYsyVqQQc3uomyt0uUA_GQEBgfTYznuyfPGxd8FzYqkBK3K5oVIOyDrcoL0gzaHgdikBAa4RUMme42bKeyFk-VyqqLPkCx8zQmB4oWHEEIpHuUglAQUSwDH34aFmRzn7SNFtkNC_16Jwb4a1nkd08VF5TtTpC-uYI6IvmSYJ7dKT1Ssk8uMSxwLSZ6BvEwHKJYvkiFUg2NsVQPCI6y9CMWXCqJ5tOiszxg0NspVeqIirAdI7lSrCEC6dorSiWhCOWBqbIwuTWG0JxbRf0ycocIL_bn8wB3uGYDCFBppUdlyeF8aLTizxdwIEZj6T0fUQKnRXT5g8AQGf1ntCuVPIcWo9lPqo6X2o5jIn7MGXk79PDhFUFt7WKwEBdGzyhvuJq52Q_NxWtbRWlaS3MwV3AHlVl3B3kEES_ygBASvjkoZC7RU35xoFHO64IPg6c0gkM_yhEftfZPQluh0RAbeKnQYvN0EJX-66eBul89XsW31TIGqowlTG01BFjD4vn07NlTCZSMYyww4uN7mRaFmcwhxYTDQrmM2flLHiJygBAQEBAQGfP1pCUlwOROtFqeGJJtcHb_EKynWbDnyrfJl5UiHfEzJ2QnMnfUrdwLZ11u5Fohfv5XafLPiEo6HEi0-IuucfAQEBpfcr70oGQ6bxfJJMoYIdJMjFi7i3v5QTRn2pcB0StCfuMMY6dMQGIhXVlprtegvZpLVf4CtbEsmNlkZgkPvfLQEBATCbkGfPJXmYLZh9f1-rbjFv9MIe6a95LYnY59XPmbEZeBfbIhvlXAXlz6BEHCTFI7YPp6tNpGWyU93dpfsHxQYBAQHqidH0Z7L0IenfN2sRvlVvkN85TzqZR_WCjLEgr_AsD5OhrP5-aJu_uaLSRrNIkn8afmBeIlzofrlcqV_nR1seAQEFAQH8O7Vl94cFJ-ZNBc_33ZVXpzbq1tVqEhdyPCRwWw9VIq7AB-ZtWeg_ECXqEHl5IeCvD8QP242QUQMXCBJTIy4uAQHaaMCPDiSXHgS_LIp0MpyHU6t2mwCFGUGlhGwcbEJvGnOUqL2N020CaLCAPUF15Zinb6IW8nXQYmLiZEJ2lGU9AQGbdpiCw8zAwluQAvZoM9npleYDyFUW1IIw481ZXOJbFz8-HIgq0mr-nLnRYdjUZVBtuYaIYQ04QLGnFrVfjBE6AQGLwg5R7sxYXxdJlI0tDgwL7tgfT5WR5Rt7rj5RogItKtaoodbHKyCfhJh2p2QIJntKav6GliWDxDYZw7wrdEkyAQFe6Xd0k6s8GC0ip6f4lhJNqn1BPF81S-il3wf3xF0qJq7My9qUxZYX9bJc9AGMD0wppdp0LNTai2W1NaxrsH0WAQHkXRP-sMZNWysvjxx2IhYuvsHk8ets0kLvSmbth0vNHuoOlLBdjdJ9bY5t3YBgZKFO2hDwQMGRUsmeJfqbHlsYAQEBETmpTk_ctjxQ90WTO4-XpvMyBBI5PQdONBINVFTWiaAQLMMB971ymI8aUfH8qCQYBYMWje0COObe5RQUCUUsnTtGzmd15bzD4b9crydQDDm9Rh7EZAfi6KaVCEZ-XVnIDIs0dwWPZlnoZ5mnkVNzxGzAWpQmEzOwmPRaVo7Ratkl2rB20vP-C60MOUWdzBxBOJt-_7EpXDmTqHb6dXVJEQqJsjdoO77zNdOfZWLmNkxSyWRzi2WL9zIVtmL4hazBHmj5cFiYs1hSIlL-OOERQw72A3IeF0Byxs6K_x907Us7W4QVhWX1W92Js51NqDwKRtOxQwgBywOVuXYvEJUNPwouMMPjOlQfJsCedQiGDTA64rlShqPmnXBSWqBiDFPQAtg0_OQJntqy7i2sXTay8J0l7FP7ZU3RP3aeLQ6mgUMIXn7dKP9t2_IhUsz8-_wzi-fjc9r93ifnkXWt4VBTxSql_CgRJy0Eiv0hyT7qKbrog1FbZnFZMXU9PdO1LDrSEgecDFJlyfdJ1NcXsxCMNTscc65vagWfloZhPsHujzYQY4E8ifUQ6t4JFgbL5CXvaxmGqhJGUmEL42D-NM5iRD37JgPubfB9FVsRnKaOHZJfA0kYdqwKHUSL05mJ9Yi2PgmajUYltW_QIHKHqzgYqji_AcTNIu3CCRU_bGqjHkARtb2CwXRr0Xgw99qUCy70eVPMGf-j7W0-yFxaH18CXQXgRYKEqIVXfNN638j0SRPNQHCNbWkcwtTk2kBzt3MeGx5zCgfx8YendUuGXlZw6TFrzJFMUyZp6KJnJ70EMeIzHZ7wMy_xFKAyTOyuEYcZZbE8Vo0_vr3N9TJNLhVAfAbyek0NE9pG0BgAJ-ves5oKXidg1dLo9BJZi6e87cEHIQKh666w-0g_ymwFfzZvWTJlT8vYhbYcl6fiQYWWmFYvovHYojyJ713XAiHj-jdwpofGwPMffcgHfNhYsdXI8AIz6vIE7sSKvqzVdBwEETGbKvFc4zaXIOPLRDxNRz3INYXUIxBjYtlogbAhSfLvZ8SqawiZdsrF1INUC1M9xgMcNTHncQOkXt5gDmeYEl4LgNzVlWv_830vJtyhBU7SIRY96i-FBsH8pv4WrmrDuCpNK8jY-Gr4tCX3It3nrRPjMdTXI9WDkaCYFD4wH4tIGiKf0J18AOYHY3j7GU_UQtYVRUsHLwL2RNEFH_kFC23kCMLUuJi4u1SICYuMCR34dwOSwsZ_RSbD40MgPHjfnKyMg-QjhkRlB59uiOb3nPJjHnSWPNEw2zKISUCKRTjqqbrGEn_syQO2ElhFnqQ7iT8FOcGY0PRYFFoHWayaYQYPVGhOMmheixy5dSAyZiWosDzswuuvM8Rgv8eRhd6W25U06Z9ydYyeMDxEd-Cnt453FWVjzxYeDBrb4u2AmWfeaMYL3Hh89YNwTEzkPnprfZAXTeOyalroTehMXHDB93Lve9y6oPfijKK0UWGojtpS6SJTMoWR-xt1vYQaK9RYoqwjDUsbNQjJA8189reG9l9lBppzM9o7Qq8C46goOwxiIY9hvJzrnvxRSy33JNm1ixkm33V27WzIW2sGd3gCeezqj_TwmwMat9BIO-Ptsfe8JhtJREpg066xqxzNVExc0AkhGydU7EqQZqxRuG-lIOqHME1uLttrFFx0EflHp7AizgQhOhDpYOH3Ke9D3tZ95AU7itWJntEx8msnSdFDyj4H_X8ao6CAgi-0yb4VvK32SDD1NkGuWTtjTlBX2VZdHZYm8Lf97MXqObN1gYEas8S0L3ZhJ8u5j567aDL9nUFTVVzYOdAySOQlxyu4KNQl4OkwgdTEzMXJNQcc294zlTA0LrUgEAQolCeXizaF7jZN5CX9BL7agtDcwaofK2pwkaFY5GSECQK-5QY5Cdcnfi9SHo6W_cxz0likUQwfIfVq5LeznnZFy2QFqROSg09s8Pgbw2Aelzme165-aJ-yNhEwNKsIhmJFVS0c3s7pGlpIjAGxCyr2nTZ7VHj7lrmU3CYMiosh-V4icMbKLe-pIDzKB0TRiDVbrdyZ1x_-sGez12L31lQfcudJ5v2ExYjDjbU9paZsmJraFSjihI3acpVGLy6ydI8PFx6svxSYbKNjPBvbDu7GIDdieNht17sr3C1JpIC-4R-7p5K1C7PYcuBTKsf9pxexFEI_WWfAeN1r1eB0hUtq44NhrAYGY2qYw-IjxtqjM5lrZ0XSIaXAQ6uuAby79b90ZDkHVkYx29FLWD3sLXHmewEyhyPES0l8TjpzaQKYkDWR8fDEHoQGq1aJBwKa1Mn8mumMfPsubyF4IyY8AbbH5NYGCwzlXlQgTSMyHBHngavy-QhYqExy1Q4QFLwS8DxzU3MTOzNec3m-8Q2H3XEE9pMnXUroNvB5J6syvXJEYRwg8YkTXyYypW1pEx0ofo-SQrRFSCRlaacHcufnQuidpZ9m2Ram5GzujrI2qxYjOBWDN7XbS3SM5pasX5fFtZjGJ6X8Z-HRK7XE0SZLEIE_Nnuc7jvFBDS5cu9xKfj-d33CHMu9ynoC4uWyOU6J_yoOxUO1WMw1J8sEfUT4MkOPqepUJfXCWIpZ5-YcCUsqFALnjMrDrocLyNNqye3zeiM0llom7GRBUARVOgu_qhSAH0FwZSIatvelXDBuApqMgFvAeggcd9JAWgU2OLVjYrRpgAyoQCzBAqlfK7LsgXmqfrtz3EqiyEkHSywoKzH-95N_3PGIjCLVgI--das5IptlDWz8-ee5L3i93zKWaV3iXdw9B7eAhwum0yw7AL2dAzE9VJgSJcGHU8kFOHj35LCcxzfzpSDSSXvx8Lgs0SpghNUavKyFS7vi7Dg5rdy37t4BqMnw8S-kwc5EHlK9QetaIv01qd52w-8tsCDkBzucUuYEOwUx6Uq2Qfnp74SlgcrFA4sMWX5hHvaTPXZgNUv9hDNtkjR4_dST8MM6CnSbv-8Jor_UVwEujxcC5HnJGFfYExH0Jm1u0jMbhmI3QbDHOQrfTax4tz8FvzYOosD9oTx3a5dzZGis0lg1LmHkSANhH6jVRqhVFtRHLa-FJjMD7gfiGsaAatUvCAIxcwSuMTWIGGoyu_Vz71IWFvJBPy3mP8Ra-0EnSoMwpJfCxhRMo4ChHp496YEIXjsBAQGNLFJlMmvlQ6YgKkOW9pIxPZotU-RsGTBsBHLYdOLoBQEBwMMUpUByzCsqdYI1wF6oEQEUigFzKgHQhpFTYxP0TikBAdVYrhqiidw3x0M84ho_ykA5Ph07lB9x51DEgWJ_1Qo-AQE_6YNIoiucwXuJv31YfNndNc8rlyL_xwZV39ZgwAi6EAEFyIoZCDiAPKraviC9UJdejvMFKbF0VZjnJpR-e_mbcwJSJiqKkHUXurAV9pX3OCHT_d7cKLPMKdk6Y0wTfeZnOjhJZ2lA1nydyKOvXFe4NbG2y-B2YLto9ltTcMBnpHESnmiOlLnJ9SBAGn53tyO8NFCbaYOotdk6kX-w025lZgEmMfMCNQnzQbGxlLEKQfgEctbYL6srOH7_XE1q1maNEgEB2L8vfqhGnOmHtnrUvbuRgI7-yehGTbmHRgh4x_v2FxsBAcR3N0sBitbxFMa4KkSgXqacq9eLEDHqjSYIRalJCyohAQHHTt8lq8MBA_YDZiTSj99YlWtuqIfXExIxjkFuwCMCAgEBAYcIdlceER2FdlvmvQF_1-W7HAWmOkuOgYJ5QWGlVcoTAQFgV1jdtyt2TFI_JxH2iPKiKxb3zcOdNhq-of3uAccsPQEB4Ur6Qh8bgnRGlcLLZO9CEI7z3SnSMIecjGpNhMN3vR8BAVCqxM0Xc-97TiJIMP1aHnyDl9FBWxDArbwMrX6Y1B09AQWNiu9uQVYKOuEGxn8RR5lKzcB4tWGBYfzqdDiGKWtuA6LdAl_GNhcaKFqcOXIlpBfWxoGrulQ1XCm2DGRfjMMCLC0fgP3HHguoR6Arm8BkACEfaZs_ab67TCLAqxJVlA-CNJifmJwzz9yddf6LOv1g6ood1Muof6Er7LxPZuUMK3Pvwy1KfrmIV0-8nsmlm3yZH4jGnL_mpHmtgsLXMFYaAQGa3zfDzMfvLJlT4JOPU035rQAOHxBbVNoyCt1NyWMEKQEBd34Ufr46TGmGtWKhOK5OeO8Abr3374M2D-mUmc_aaTkBAbD8EL0Bt2M-EUCmKUd3EzbN_7BHivzQedBvurXzOXAD
        */

        assert!(
            &StagedLedgerDiffTuple::from(et.0.t.staged_ledger_diff.t.diff.clone())
                .diff_one()
                .is_none()
        );
        let commands = StagedLedgerDiffTuple::from(et.0.t.staged_ledger_diff.t.diff.clone())
            .diff_two()
            .commands
            .clone();
        assert_eq!(commands.len(), 3);

        match &commands[0].data {
            UserCommand::SignedCommand(command) => {
                let bytes = bs58::decode("B62qoSuxNqwogusxxZbs3gpJUxCCN4GZEv21FX8S2DtNpToLgKnrexM")
                    .into_vec()
                    .unwrap();
                assert_eq!(command.signer.x.to_bytes(), bytes[3..35]);

                assert_eq!(Signature::from(command.signature.clone()).to_base58_string(), "7mXTB1bcHYLJTmTfMtTboo4FSGStvera3z2wd6qjSxhpz1hZFMZZjcyaWAFEmZhgbq6DqVqGsNodnYKsCbMAq7D8yWo5bRSd");
                let bytes = bs58::decode("7mXTB1bcHYLJTmTfMtTboo4FSGStvera3z2wd6qjSxhpz1hZFMZZjcyaWAFEmZhgbq6DqVqGsNodnYKsCbMAq7D8yWo5bRSd")
                    .into_vec()
                    .unwrap();
                assert_eq!(command.signature.field_point().as_ref(), &bytes[2..34]);
                assert_eq!(
                    command.signature.inner_curve_scalar().as_ref(),
                    &bytes[34..66]
                );

                assert_eq!(command.payload.common.nonce.0, 5694);
                assert_eq!(
                    command.payload.common.memo.0,
                    SignedCommandMemo::try_from("FPayment").unwrap().0,
                );
                assert_eq!(command.payload.common.fee.to_string(), "0.010000000");
                assert_eq!(command.payload.common.fee_token.0, 1);
                // FIXME: Fix valid_util (Extended_U32)
                // assert_eq!(command.payload.common.valid_until.0, 4294967295);
                match &command.payload.body {
                    SignedCommandPayloadBody::PaymentPayload(body) => {
                        assert_eq!(body.amount.to_string(), "0.027370000");
                        let bytes =
                            bs58::decode("B62qoSuxNqwogusxxZbs3gpJUxCCN4GZEv21FX8S2DtNpToLgKnrexM")
                                .into_vec()
                                .unwrap();
                        // TODO: Validate full bytes vec with salted mainnet signature
                        assert_eq!(body.source_pk.x.to_bytes(), bytes[3..35]);
                        let bytes =
                            bs58::decode("B62qn2MtuQ9GyyVnotUHB9Ehp9EZre5m6TYpGx64tBCDHHBZFZRURnL")
                                .into_vec()
                                .unwrap();
                        // TODO: Validate full bytes vec with salted mainnet signature
                        assert_eq!(body.receiver_pk.x.to_bytes(), bytes[3..35]);
                        assert_eq!(body.token_id.0, 1);
                    }
                    _ => bail!(
                        "PaymentPayload expected, but found: {:#?}",
                        command.payload.body
                    ),
                };
            }
            _ => bail!("SignedCommand expected, but found: {:#?}", commands[0].data),
        }

        match &commands[0].status {
            TransactionStatus::Applied(applied) => {
                let auxiliary_data = applied.auxiliary_data();
                assert!(auxiliary_data.fee_payer_account_creation_fee_paid.is_none());
                assert!(auxiliary_data.receiver_account_creation_fee_paid.is_none());
                assert!(auxiliary_data.created_token.is_none());

                let balance_data = applied.balance_data();
                assert!(balance_data.fee_payer_balance.is_some());
                assert_eq!(balance_data.fee_payer_balance.unwrap().0, 59778375293571);
                assert!(balance_data.source_balance.is_some());
                assert_eq!(balance_data.source_balance.unwrap().0, 59778375293571);
                assert!(balance_data.receiver_balance.is_some());
                assert_eq!(balance_data.receiver_balance.unwrap().0, 11241317900);
            }
            _ => bail!(
                "TransactionStatus::Applied expected, but found: {:#?}",
                commands[0].status
            ),
        }

        let coinbase = StagedLedgerDiffTuple::from(et.0.t.staged_ledger_diff.t.diff.clone())
            .diff_two()
            .coinbase
            .clone();
        match coinbase {
            CoinBase::One(_) => {}
            _ => bail!("CoinBase::One expected, but found: {:#?}", coinbase),
        };

        let internal_commands =
            StagedLedgerDiffTuple::from(et.0.t.staged_ledger_diff.t.diff.clone())
                .diff_two()
                .internal_command_balances
                .clone();
        assert_eq!(internal_commands.len(), 2);
        match &internal_commands[0] {
            InternalCommandBalanceData::CoinBase(cb) => {
                assert!(cb.fee_transfer_receiver_balance.is_none());
                assert_eq!(cb.coinbase_receiver_balance.0, 20203793056339);
            }
            _ => {
                bail!("CoinBase expected, but found: {:#?}", internal_commands[0])
            }
        };

        match &internal_commands[1] {
            InternalCommandBalanceData::FeeTransfer(ft) => {
                assert_eq!(ft.receiver1_balance.0, 20203805056339);
                assert!(ft.receiver2_balance.is_none());
            }
            _ => {
                bail!(
                    "FeeTransfer expected, but found: {:#?}",
                    internal_commands[1]
                )
            }
        };

        let bytes = bs58::decode("jwHLk8kaC6B45K3sjuX2sM38649VtfpUAteTfKFQMPcqTeXjGiT")
            .into_vec()
            .unwrap();
        assert_eq!(
            et.0.t.delta_transition_chain_proof.0.t.as_ref()[..],
            bytes[2..34]
        );
        // FIXME: Version byte here disagrees with what is being used for genesis block
        // Note that version byte is not part of binprot binary, it's only used for bs58 representation
        // assert_eq!(
        //     et.delta_transition_chain_proof.0.into(.clone()).to_base58_string(),
        //     "jwHLk8kaC6B45K3sjuX2sM38649VtfpUAteTfKFQMPcqTeXjGiT"
        // );
        assert_eq!(et.0.t.current_protocol_version.t.major, 2);
        assert_eq!(et.0.t.current_protocol_version.t.minor, 0);
        assert_eq!(et.0.t.current_protocol_version.t.patch, 0);
        assert_eq!(
            et.0.t.current_protocol_version,
            ProtocolVersion::default().into()
        );
        assert_eq!(et.0.t.proposed_protocol_version_opt, None);
        Ok(())
    }
}
