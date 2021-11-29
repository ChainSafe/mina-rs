// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use mina_rs_base::types::*;

/// Trait for genesis block initialization logic
/// # Example
/// ```
/// use mina_rs_base::types::*;
/// use mina_consensus::genesis::{GenesisInit, DEVNET_CONFIG, MAINNET_CONFIG};
/// let genesis_mainnet = ExternalTransition::init_genesis(&MAINNET_CONFIG);
/// let genesis_devnet = ExternalTransition::init_genesis(&DEVNET_CONFIG);
/// ```
pub trait GenesisInit {
    fn init_genesis(config: &GenesisInitConfig) -> ExternalTransition;
}

impl GenesisInit for ExternalTransition {
    /// Initialize genesis block
    /// <https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#611-genesis-block>
    fn init_genesis(config: &GenesisInitConfig) -> ExternalTransition {
        let mut et = ExternalTransition::default();

        let cs = &mut et.protocol_state.body.consensus_state;
        cs.blockchain_length = 1.into();
        cs.epoch_count = 0.into();
        cs.min_window_density =
            (config.sub_windows_per_window * config.constants.slots_per_sub_window.0).into();
        cs.sub_window_densities = vec![
            1.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
            7.into(),
        ];
        cs.last_vrf_output = VrfOutputTruncated(vec![0; 32]);
        cs.total_currency = Amount(805385692840039233);
        cs.curr_global_slot = GlobalSlot {
            slot_number: 0.into(),
            slots_per_epoch: config.constants.slots_per_epoch,
        };
        cs.global_slot_since_genesis = 0.into();
        cs.staking_epoch_data = config.staking_epoch_data.clone();
        cs.next_epoch_data = config.next_epoch_data.clone();

        et
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_crypto::base58::Base58Encodable;

    // https://github.com/MinaProtocol/mina/tree/feature/9665-spec-ouroboros-samasika-checkpointing/docs/specs/consensus#611-genesis-block
    #[test]
    fn test_genesis_mainnet() {
        let et = ExternalTransition::init_genesis(&MAINNET_CONFIG);

        let cs = &et.protocol_state.body.consensus_state;
        assert_eq!(cs.blockchain_length, 1.into());
        assert_eq!(cs.epoch_count, 0.into());
        assert_eq!(cs.min_window_density, 77.into());
        assert_eq!(cs.sub_window_densities.len(), 11);
        assert_eq!(
            cs.sub_window_densities(),
            vec![1, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]
        );
        assert_eq!(cs.last_vrf_output.0, vec![0; 32]);
        assert_eq!(
            cs.total_currency.to_formatted_string(),
            "805385692.840039233"
        );
        assert_eq!(cs.curr_global_slot.slot_number, 0.into());
        assert_eq!(cs.curr_global_slot.slots_per_epoch, 7140.into());
        assert_eq!(cs.global_slot_since_genesis, 0.into());

        {
            let sed = &cs.staking_epoch_data;
            assert_eq!(
                sed.ledger.hash.to_base58().into_string(),
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
            );
            assert_eq!(sed.ledger.total_currency.0, 805385692840039300);
            assert_eq!(
                sed.seed.to_base58().into_string(),
                "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA"
            );
            assert_eq!(
                sed.start_checkpoint.to_base58().into_string(),
                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
            );
            assert_eq!(
                sed.lock_checkpoint.to_base58().into_string(),
                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
            );
            assert_eq!(sed.epoch_length, 1.into());
        }
        {
            let ned = &cs.next_epoch_data;
            assert_eq!(
                ned.ledger.hash.to_base58().into_string(),
                "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee"
            );
            assert_eq!(ned.ledger.total_currency.0, 805385692840039300);
            assert_eq!(
                ned.seed.to_base58().into_string(),
                "2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt"
            );
            assert_eq!(
                ned.start_checkpoint.to_base58().into_string(),
                "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
            );
            assert_eq!(
                ned.lock_checkpoint.to_base58().into_string(),
                "3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d"
            );
            assert_eq!(ned.epoch_length, 2.into());
        }
    }
}
