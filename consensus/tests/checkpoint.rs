// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_consensus::{checkpoint::*, common::*, error::ConsensusError};
    use mina_crypto::{hash::*, prelude::*};
    use mina_rs_base::types::*;
    use wasm_bindgen_test::*;

    pub fn init_checkpoints(genesis: &mut ProtocolState) -> Result<(), ConsensusError> {
        genesis.body.consensus_state.staking_epoch_data.seed = EpochSeed::default();
        genesis
            .body
            .consensus_state
            .staking_epoch_data
            .start_checkpoint = StateHash::default();
        genesis
            .body
            .consensus_state
            .staking_epoch_data
            .lock_checkpoint = StateHash::default();
        genesis
            .body
            .consensus_state
            .staking_epoch_data
            .epoch_length
            .0 = 1;
        genesis.body.consensus_state.next_epoch_data.seed =
            EpochSeed::from_base58("2vaRh7FQ5wSzmpFReF9gcRKjv48CcJvHs25aqb3SSZiPgHQBy5Dt")
                .map_err(|_| ConsensusError::ConsensusStateNotFound)?;
        genesis
            .body
            .consensus_state
            .next_epoch_data
            .start_checkpoint = StateHash::default();
        genesis.body.consensus_state.next_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d")
                .map_err(|_| ConsensusError::ConsensusStateNotFound)?;
        genesis.body.consensus_state.next_epoch_data.epoch_length.0 = 2;
        Ok(())
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_init_checkpoints() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();
        assert_eq!(
            genesis
                .body
                .consensus_state
                .staking_epoch_data
                .start_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            genesis
                .body
                .consensus_state
                .staking_epoch_data
                .lock_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            genesis
                .body
                .consensus_state
                .next_epoch_data
                .start_checkpoint,
            StateHash::default()
        );
        assert_eq!(
            genesis.body.consensus_state.next_epoch_data.lock_checkpoint,
            StateHash::from_base58("3NLoKn22eMnyQ7rxh5pxB6vBA3XhSAhhrf7akdqS6HbAKD14Dh1d").unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_is_short_range() {
        let mut genesis: ProtocolState = Default::default();
        init_checkpoints(&mut genesis).unwrap();
        let mut c0: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c1: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut c3: ProtocolStateChain = ProtocolStateChain(vec![]);
        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        b0.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(7140),
        };
        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1),
            slots_per_epoch: Length(7140),
        };

        c0.push(b0).unwrap();
        c1.push(b1).unwrap();
        assert_eq!(is_short_range(&c0, &c1).unwrap(), true);
        assert_eq!(is_short_range(&c1, &c0).unwrap(), true);

        init_checkpoints(&mut genesis).unwrap();
        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(2);
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(2),
            slots_per_epoch: Length(7140),
        };
        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(667);
        b2.body.consensus_state.epoch_count = Length(11);
        b2.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(667),
            slots_per_epoch: Length(7140),
        };
        c3.push(b2).unwrap();
        assert_eq!(is_short_range(&c3, &c0).unwrap(), false);
        assert_eq!(is_short_range(&c0, &c3).unwrap(), false);
    }
}
