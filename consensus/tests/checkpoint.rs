// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_consensus::{
        common::*,
        error::ConsensusError,
        genesis::{Genesis, MAINNET_CONFIG},
    };
    use mina_crypto::{hash::*, prelude::*};
    use mina_rs_base::types::*;
    use wasm_bindgen_test::*;

    fn genesis_consensus_state() -> ProtocolStateChain {
        let genesis = ExternalTransition::from_genesis_config(&MAINNET_CONFIG);
        let a = genesis.protocol_state;
        ProtocolStateChain(vec![a])
    }

    #[test]
    #[wasm_bindgen_test]
    fn short_range_same_chain_same_epoch() {
        let chain_a = genesis_consensus_state();
        let chain_b = genesis_consensus_state();
        assert!(chain_a.is_short_range(&chain_b).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    #[should_panic]
    fn short_range_fails_when_chains_in_different_epoch_and_different_lock_checkpoints() {
        let mut chain_a = genesis_consensus_state();
        let a = &mut chain_a.0[0];
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

        let mut chain_b = genesis_consensus_state();
        let b = &mut chain_b.0[0];
        // setting states to block at height 5076
        // block in json: https://storage.googleapis.com/mina_network_blocfk_data/mainnet-3NLC8CV9kZYFkXnUipJzkkHvT9RmsttSUNpwfwqwWCfbP9bQmwNJ.json
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

        assert!(chain_a.is_short_range(&chain_b).unwrap());

        // Epoch = 0

        // {
        //     "blockHeight": 5075,
        //     "creatorAccount": {
        //       "publicKey": "B62qqhURJQo3CvWC3WFo9LhUhtcaJWLBcJsaA3DXaU2GH5KgXujZiwB"
        //     },
        //     "dateTime": "2021-03-31T20:57:00Z",
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

        // Epoch = 1
        //   {
        //     "blockHeight": 5076,
        //     "creatorAccount": {
        //       "publicKey": "B62qqhURJQo3CvWC3WFo9LhUhtcaJWLBcJsaA3DXaU2GH5KgXujZiwB"
        //     },
        //     "dateTime": "2021-03-31T21:00:00Z",
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
    }
}
