// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! The following module tests the short range fork rule for
//! mina consensus for the following cases:
//! * Same chain
//! * Chains that are more than 1 epoch apart
//! * Chains that are 1 epoch apart but beyond the seed update range.
//! * Chains that are 1 epoch apart but have same lock checkpoint
//!

#[cfg(test)]
mod tests {
    use mina_consensus::{
        common::*,
        genesis::{Genesis, MAINNET_CONFIG},
    };
    use mina_crypto::hash::*;
    use mina_rs_base::types::*;
    use wasm_bindgen_test::*;

    /// helper to initialize consensus state to defaults for tests
    fn genesis_consensus_state() -> ProtocolStateChain {
        let genesis = ExternalTransition::from_genesis_config(&MAINNET_CONFIG);
        let a = genesis.protocol_state;
        ProtocolStateChain(vec![a])
    }

    #[test]
    #[wasm_bindgen_test]
    fn is_short_range_when_both_same_chain() {
        let chain_a = genesis_consensus_state();
        let chain_b = genesis_consensus_state();
        assert!(chain_a.is_short_range(&chain_b).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    fn short_range_fails_when_chains_are_one_epoch_apart_but_beyond_seed_update_range() {
        let mut chain_a = genesis_consensus_state();
        let a = &mut chain_a.0[0];

        // A block at height 5076
        // block in json: https://storage.googleapis.com/mina_network_blocfk_data/mainnet-3NLC8CV9kZYFkXnUipJzkkHvT9RmsttSUNpwfwqwWCfbP9bQmwNJ.json
        let mut chain_b = genesis_consensus_state();
        let b = &mut chain_b.0[0];
        b.body.consensus_state.epoch_count = Length(1);
        b.body.consensus_state.next_epoch_data = EpochData::default();
        a.body.consensus_state.curr_global_slot.slot_number = GlobalSlotNumber(7140);
        b.body.consensus_state.next_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW").unwrap();
        b.body.consensus_state.next_epoch_data.start_checkpoint =
            StateHash::from_base58("3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW").unwrap();

        b.body.consensus_state.staking_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn").unwrap();
        b.body.consensus_state.staking_epoch_data.start_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();

        assert!(!chain_a.is_short_range(&chain_b).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    fn different_epoch_same_lock_checkpoint() {
        let mut chain_a = genesis_consensus_state();
        let a = &mut chain_a.0[0];
        // the block at heigh 5075
        // {
        //     "blockHeight": 5075,
        //     "protocolState": {
        //       "consensusState": {
        //         "blockHeight": 5075,
        //         "epochCount": 0,
        //         "nextEpochData": {
        //           "lockCheckpoint": "3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn",
        //           "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
        //         },
        //         "stakingEpochData": {
        //           "lockCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
        //           "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
        //         }
        //       }
        //     },
        //     "stateHash": "3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW"
        //   },
        a.body.consensus_state.epoch_count = Length(0);
        a.body.consensus_state.next_epoch_data = EpochData::default();
        a.body.consensus_state.next_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn").unwrap();
        a.body.consensus_state.next_epoch_data.start_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();

        a.body.consensus_state.staking_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();
        a.body.consensus_state.staking_epoch_data.start_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();
        a.body.consensus_state.curr_global_slot = GlobalSlot::default();
        a.body.consensus_state.curr_global_slot.slot_number = GlobalSlotNumber(7139);
        a.body.consensus_state.curr_global_slot.slots_per_epoch = Length(7140);

        let mut chain_b = genesis_consensus_state();
        let b = &mut chain_b.0[0];
        // the block at heigh 5075
        //   {
        //     "blockHeight": 5076,
        //     "protocolState": {
        //       "consensusState": {
        //         "blockHeight": 5076,
        //         "epochCount": 1,
        //         "nextEpochData": {
        //           "lockCheckpoint": "3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW",
        //           "startCheckpoint": "3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW"
        //         },
        //         "stakingEpochData": {
        //           "lockCheckpoint": "3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn",
        //           "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
        //         }
        //       }
        //     },
        //     "stateHash": "3NLC8CV9kZYFkXnUipJzkkHvT9RmsttSUNpwfwqwWCfbP9bQmwNJ"
        //   },

        b.body.consensus_state.epoch_count = Length(1);
        b.body.consensus_state.next_epoch_data = EpochData::default();
        b.body.consensus_state.next_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW").unwrap();
        b.body.consensus_state.next_epoch_data.start_checkpoint =
            StateHash::from_base58("3NKmKfm2RSTfA1w5mNSJRLoyAQgcRhWjH5qdNynchHar4kBmJPbW").unwrap();

        b.body.consensus_state.staking_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn").unwrap();
        b.body.consensus_state.staking_epoch_data.start_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();
        b.body.consensus_state.curr_global_slot = GlobalSlot::default();
        b.body.consensus_state.curr_global_slot.slot_number = GlobalSlotNumber(7140);
        b.body.consensus_state.curr_global_slot.slots_per_epoch = Length(7140);

        assert!(chain_a.is_short_range(&chain_b).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    fn short_range_fails_when_more_than_one_epoch_apart() {
        let mut chain_a = genesis_consensus_state();
        let a = &mut chain_a.0[0];
        // block at 4865
        // {
        //     "blockHeight": 4856,
        //     "protocolState": {
        //       "consensusState": {
        //         "blockHeight": 4856,
        //         "epochCount": 0,
        //         "nextEpochData": {
        //           "lockCheckpoint": "3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn",
        //           "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
        //         },
        //         "stakingEpochData": {
        //           "lockCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
        //           "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"
        //         }
        //       }
        //     },
        //     "stateHash": "3NL78w7jWqWuEtQKb2UkeZUsxdGGyL8FUj4pQLs5pkaMUUb2SfD7"
        //   },
        a.body.consensus_state.epoch_count = Length(0);
        a.body.consensus_state.next_epoch_data = EpochData::default();
        a.body.consensus_state.next_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NLWRuFB7G8CPkizXnRwpAUcQu5cAS5RTWE5vhWL1XBE47oEJ2kn").unwrap();
        a.body.consensus_state.next_epoch_data.start_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();

        a.body.consensus_state.staking_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();
        a.body.consensus_state.staking_epoch_data.start_checkpoint =
            StateHash::from_base58("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap();
        a.body.consensus_state.curr_global_slot = GlobalSlot::default();
        a.body.consensus_state.curr_global_slot.slot_number = GlobalSlotNumber(6839);
        a.body.consensus_state.curr_global_slot.slots_per_epoch = Length(7140);

        // block at height 107697
        // {
        //     "blockHeight": 107697,
        //     "dateTime": "2022-02-10T12:51:00Z",
        //     "protocolState": {
        //       "consensusState": {
        //         "epochCount": 22,
        //         "nextEpochData": {
        //           "lockCheckpoint": "3NKPqRTPrJJVyrb4RKMikfJG9WabCV1fH6aw2p62dEKNpf1aVeTy",
        //           "startCheckpoint": "3NLuG9aJEm4xm4tNEzYKM88aNksquUvPbmdACgf7kQnhCBEcE2gJ"
        //         },
        //         "slot": 1577,
        //         "stakingEpochData": {
        //           "lockCheckpoint": "3NKW9MqYePrfK48ZAn3iu7iDDc17wZkTmzo5tHQ96bvG1gUfG9Dv",
        //           "startCheckpoint": "3NKcySpVjMsrLpCHYvzmEvap4HUhSdws8HU7fv5cL3wgamYVzQts"
        //         }
        //       }
        //     },
        //     "stateHash": "3NLeVgZfjMpFwMJ5QHLdWvxERMY9cvwQsYRBkNudn8rTXucxyVwz"
        //   }
        let mut chain_b = genesis_consensus_state();
        let b = &mut chain_b.0[0];
        b.body.consensus_state.epoch_count = Length(22);
        b.body.consensus_state.next_epoch_data = EpochData::default();
        b.body.consensus_state.next_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NKPqRTPrJJVyrb4RKMikfJG9WabCV1fH6aw2p62dEKNpf1aVeTy").unwrap();
        b.body.consensus_state.next_epoch_data.start_checkpoint =
            StateHash::from_base58("3NLuG9aJEm4xm4tNEzYKM88aNksquUvPbmdACgf7kQnhCBEcE2gJ").unwrap();

        b.body.consensus_state.staking_epoch_data.lock_checkpoint =
            StateHash::from_base58("3NKW9MqYePrfK48ZAn3iu7iDDc17wZkTmzo5tHQ96bvG1gUfG9Dv").unwrap();
        b.body.consensus_state.staking_epoch_data.start_checkpoint =
            StateHash::from_base58("3NKcySpVjMsrLpCHYvzmEvap4HUhSdws8HU7fv5cL3wgamYVzQts").unwrap();
        b.body.consensus_state.curr_global_slot = GlobalSlot::default();
        b.body.consensus_state.curr_global_slot.slot_number = GlobalSlotNumber(1577);
        b.body.consensus_state.curr_global_slot.slots_per_epoch = Length(7140);

        assert!(!chain_a.is_short_range(&chain_b).unwrap());
    }
}
