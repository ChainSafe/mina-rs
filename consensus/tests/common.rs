// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use hex::ToHex;
    use mina_consensus::{common::*, error::ConsensusError};
    use mina_crypto::hash::*;
    use mina_rs_base::types::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_push() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(c.length(), 0);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();
        assert_eq!(c.length(), 1);

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        c.push(b1).unwrap();
        assert_eq!(c.length(), 2);

        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(2);
        c.push(b2).unwrap();
        assert_eq!(c.length(), 3);

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        assert_eq!(c.push(b1).unwrap_err(), ConsensusError::InvalidHeight,);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_top() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(c.length(), 0);
        assert_eq!(c.top(), None);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();
        assert_eq!(c.length(), 1);
        let expected: ProtocolState = Default::default();
        assert_eq!(c.top(), Some(&expected));

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        c.push(b1).unwrap();
        let mut expected: ProtocolState = Default::default();
        expected.body.consensus_state.blockchain_length = Length(1);
        assert_eq!(c.top(), Some(&expected));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_epoch_slot() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        b0.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(1000),
        };
        c.push(b0).unwrap();
        let epoch_slot = c.epoch_slot();
        assert_eq!(epoch_slot, Some(0));

        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1),
            slots_per_epoch: Length(1000),
        };
        c.push(b1).unwrap();
        let epoch_slot = c.epoch_slot();
        assert_eq!(epoch_slot, Some(1));

        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(2);
        b2.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1002),
            slots_per_epoch: Length(1000),
        };
        c.push(b2).unwrap();
        let epoch_slot = c.epoch_slot();
        assert_eq!(epoch_slot, Some(2));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_state_hash() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0).unwrap();

        let hash = c.state_hash();
        hash.unwrap();
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_last_vrf() {
        let mut c: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(None, c.last_vrf_hash());

        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        c.push(b0.clone()).unwrap();

        let expected = Some(
            b0.body
                .consensus_state
                .last_vrf_output
                .hash()
                .as_ref()
                .encode_hex(),
        );
        assert_eq!(expected, c.last_vrf_hash());
    }

    #[test]
    #[wasm_bindgen_test]
    fn selects_longer_chain() {
        let mut genesis_chain = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.min_window_density = Length(77);
        consensus_state.sub_window_densities = vec![
            Length(1),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
        ];

        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(7140),
        };

        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        genesis_chain.push(prot_state).unwrap();

        let mut chain_at_5001 = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.min_window_density = Length(43);
        let densities = vec![
            Length(5),
            Length(5),
            Length(2),
            Length(5),
            Length(3),
            Length(1),
            Length(5),
            Length(3),
            Length(7),
            Length(6),
            Length(5),
        ];

        consensus_state.sub_window_densities = densities.clone();

        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(7042),
            slots_per_epoch: Length(7140),
        };

        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_at_5001.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_at_5001);
        let canonical = genesis_chain.select_secure_chain(&chains).unwrap();
        let canonical = canonical.0.get(0).unwrap();
        assert_eq!(
            canonical.body.consensus_state.min_window_density,
            Length(43)
        );
        assert_eq!(
            canonical.body.consensus_state.sub_window_densities,
            densities
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_longer_chain_with_diff_blockchain_length() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let select_result = chain_a.select_longer_chain(&chain_b).unwrap();

        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    //Same BlockChain length but different last vrf output
    //Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json
    //Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    fn test_longer_chain_with_same_chain_length_diff_last_vrf_output() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        consensus_state.last_vrf_output =
            VrfOutputTruncated::from("r0K80Xsb44NLx_pBjI9UQtt6a1N-RWym8VxVTY4pAAA=");
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        consensus_state.last_vrf_output =
            VrfOutputTruncated::from("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=");
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let select_result = chain_a.select_longer_chain(&chain_b).unwrap();

        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.last_vrf_output,
            VrfOutputTruncated::from("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=")
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_select_secure_chain_short_range_fork() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(5);
        consensus_state.blockchain_length = Length(11);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(5);
        consensus_state.blockchain_length = Length(10);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(11)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    //Long range fork with greater relative min window density
    fn test_select_secure_chain_long_range_fork_greater_relative_min_window_density() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(23);
        consensus_state.min_window_density = Length(77);
        consensus_state.sub_window_densities = vec![
            Length(1),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(1);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(14);
        consensus_state.sub_window_densities = vec![
            Length(7),
            Length(2),
            Length(2),
            Length(5),
            Length(6),
            Length(7),
            Length(5),
            Length(7),
            Length(5),
            Length(5),
            Length(5),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(167176),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(77748)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    ///Long range fork with same relative min window density
    fn test_select_secure_chain_long_range_fork_same_relative_min_window_density() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(23);
        consensus_state.min_window_density = Length(14);
        consensus_state.sub_window_densities = vec![
            Length(7),
            Length(2),
            Length(2),
            Length(5),
            Length(6),
            Length(7),
            Length(5),
            Length(7),
            Length(5),
            Length(5),
            Length(5),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(167176),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(1);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(14);
        consensus_state.sub_window_densities = vec![
            Length(7),
            Length(2),
            Length(2),
            Length(5),
            Length(6),
            Length(7),
            Length(5),
            Length(7),
            Length(5),
            Length(5),
            Length(5),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(167176),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(77748)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    //Long range fork with lesser relative min window density
    fn test_select_secure_chain_long_range_fork_lesser_relative_min_window_density() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(14);
        consensus_state.sub_window_densities = vec![
            Length(7),
            Length(2),
            Length(2),
            Length(5),
            Length(6),
            Length(7),
            Length(5),
            Length(7),
            Length(5),
            Length(5),
            Length(5),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(167176),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(23);
        consensus_state.min_window_density = Length(77);
        consensus_state.sub_window_densities = vec![
            Length(1),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
            Length(7),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(10),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(1);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(77748)
        );
    }
}
