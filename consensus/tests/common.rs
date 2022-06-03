// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use mina_consensus::{common::*, error::ConsensusError};
    use mina_rs_base::types::ExternalTransition;
    use mina_rs_base::{types::*, JsonSerializationType};
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
    // Current chain: Chain A (Long Chain)
    // Candidate chains: [Chain B]
    fn test_longer_chain_with_shorter_candidate_chain() {
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
        assert_eq!(result_state, chain_a.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain: Chain A
    // Candidate chains: [Chain B] (Long Chain)
    fn test_longer_chain_with_longer_candidate_chain() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let select_result = chain_a.select_longer_chain(&chain_b).unwrap();

        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
        assert_eq!(result_state, chain_b.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Same BlockChain length but greater last vrf output of candidate chain
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json (non-canonical)
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Current chain: Chain A
    // Candidate chains: [Chain B] (Greater Last VRF output)
    fn test_longer_chain_with_same_chain_length_greater_last_vrf_output() {
        use std::str::FromStr;
        // Chain A
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);

        // Chain A vrf hex: "dd55ef09c0474817a64efffa7fe5a3aedd2db04a5f66e52e9630b59711f56613"
        consensus_state.last_vrf_output =
            VrfOutputTruncated::from_str("r0K80Xsb44NLx_pBjI9UQtt6a1N-RWym8VxVTY4pAAA=").unwrap();
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        // Chain B
        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        // Chain B vrf hash hex: "e907e63d043c78b3dfa724b2ddc1152114fc91b983b40581b1036a8d19eb136d"
        consensus_state.last_vrf_output =
            VrfOutputTruncated::from_str("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=").unwrap(); // smaller
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let select_result = chain_a.select_longer_chain(&chain_b).unwrap();

        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.last_vrf_output,
            VrfOutputTruncated::from_str("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=").unwrap()
        );
        assert_eq!(result_state, chain_b.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Same BlockChain length but lesser last vrf output of candidate chain
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json (non-canonical)
    // Current chain: Chain A
    // Candidate chains: [Chain B] (Lesser Last VRF output)
    fn test_longer_chain_with_same_chain_length_lesser_last_vrf_output() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        // Chain A vrf hash hex: "e907e63d043c78b3dfa724b2ddc1152114fc91b983b40581b1036a8d19eb136d"
        consensus_state.last_vrf_output =
            VrfOutputTruncated::from_str("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=").unwrap();
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.blockchain_length = Length(113267);
        // Chain B vrf hex: "dd55ef09c0474817a64efffa7fe5a3aedd2db04a5f66e52e9630b59711f56613"
        consensus_state.last_vrf_output =
            VrfOutputTruncated::from_str("r0K80Xsb44NLx_pBjI9UQtt6a1N-RWym8VxVTY4pAAA=").unwrap();

        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let select_result = chain_a.select_longer_chain(&chain_b).unwrap();

        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.last_vrf_output,
            VrfOutputTruncated::from_str("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=").unwrap()
        );
        assert_eq!(result_state, chain_a.0.get(0).unwrap());
    }

    /* TODO: Blocked due to poseidon hash implementation
    Same BlockChain length and equal last vrf output of candidate chain but different state hash
    //Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-117896-3NKrv92FYZFHRNUJxiP7VGeRx3MeDY2iffFjUWXTPoXJorsS63ba.json
    //Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-117896-3NLPBDTckSdjcUFcQiE9raJsyzB84KayMPKi4PmwNybnA6J75GoL.json
    //Current chain: Chain A
    //Candidate chains: [Chain B] (Lesser State Hash)
    */

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are short range fork and Chain A is Longer than Chain B
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
        assert_eq!(result_state, chain_a.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are long range fork
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Current chain: Chain A
    // Candidate chains: [Chain B] (Greater relative min window density)
    fn test_select_secure_chain_long_range_fork_greater_relative_min_window_density() {
        let mut chain_a = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(33);
        consensus_state.sub_window_densities = vec![
            Length(6),
            Length(1),
            Length(3),
            Length(5),
            Length(4),
            Length(3),
            Length(5),
            Length(7),
            Length(4),
            Length(5),
            Length(6),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(111965),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
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
        consensus_state.blockchain_length = Length(113267);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let mut candidate_chains = vec![];
        candidate_chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&candidate_chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
        assert_eq!(result_state, candidate_chains[0].0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are long range fork, with same relative min window density
    // Current chain: Chain A
    // Candidate chains: [Chain B]
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

        let mut candidate_chains = vec![];
        candidate_chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&candidate_chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(77748)
        );
        assert_eq!(
            result_state,
            chain_a
                .select_longer_chain(&candidate_chains[0])
                .unwrap()
                .0
                .get(0)
                .unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are long range fork
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Current chain: Chain A
    // Candidate chains: Chain B (Lesser relative min window density)
    fn test_select_secure_chain_long_range_fork_lesser_relative_min_window_density() {
        // Chain A
        let json_block = test_fixtures::JSON_TEST_BLOCKS
            .get("mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json")
            .unwrap();
        let json_value: <ExternalTransition as JsonSerializationType>::T =
            serde_json::from_value(json_block.clone()).unwrap();
        let block_from_json: ExternalTransition = json_value.into();
        let mut chain_a = ProtocolStateChain::default();
        chain_a.push(block_from_json.protocol_state).unwrap();

        // Chain B
        let json_block = test_fixtures::JSON_TEST_BLOCKS
            .get("mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json")
            .unwrap();
        let json_value: <ExternalTransition as JsonSerializationType>::T =
            serde_json::from_value(json_block.clone()).unwrap();
        let block_from_json: ExternalTransition = json_value.into();
        let mut chain_b = ProtocolStateChain::default();
        chain_b.push(block_from_json.protocol_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_b);
        let select_result = chain_a.select_secure_chain(&chains).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
        assert_eq!(result_state, chain_a.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are long range fork
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Current chain: Chain A
    // Candidate chains: Chain B (with extra sub window densities)
    fn adversary_test_select_secure_chain_candidate_with_extra_sub_window_densities() {
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
        consensus_state.blockchain_length = Length(113267);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(33);
        consensus_state.sub_window_densities = vec![
            Length(6),
            Length(1),
            Length(3),
            Length(5),
            Length(4),
            Length(3),
            Length(5),
            Length(7),
            Length(4),
            Length(5),
            Length(6),
            // Extra Sub Window Density which will result in greater relative min window density and incorrect chain selection
            Length(6),
            Length(6),
            Length(6),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(111965),
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
            Length(113267)
        );
        assert_eq!(result_state, chain_a.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are long range fork
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Current chain: Chain A
    // Candidate chains: Chain B (With extremely high sub window densities)
    fn adversary_test_select_secure_chain_with_extremely_high_sub_window_densities() {
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
        consensus_state.blockchain_length = Length(113267);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(33);
        consensus_state.sub_window_densities = vec![
            Length(6),
            Length(1),
            Length(3),
            Length(5),
            Length(4),
            // changed window density of 3 to 999 impossibly high sub window densities which will result in greater relative min window density and incorrect chain selection
            Length(999),
            Length(5),
            Length(7),
            Length(4),
            Length(5),
            Length(6),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(167168), // changed to 167168 from 111965 to meet adversary test case
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

        assert_eq!(result_state, chain_a.0.get(0).unwrap());
    }

    #[test]
    #[wasm_bindgen_test]
    // Current chain and candidate chain are long range fork
    // Chain A: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Chain B: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Current chain: Chain A
    // Candidate chains: Chain B (Having very less number of sub window densities)
    // Result: Should pick up the chain with valid sub window density.
    fn adversary_test_select_secure_chain_with_less_number_of_sub_window_densities() {
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
        consensus_state.blockchain_length = Length(113267);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_a.push(prot_state).unwrap();

        let mut chain_b = ProtocolStateChain::default();
        let mut consensus_state = ConsensusState::default();
        consensus_state.epoch_count = Length(15);
        consensus_state.min_window_density = Length(33);
        consensus_state.sub_window_densities = vec![
            Length(6),
            Length(1),
            Length(3),
            // Below sub window densities are delibrately removed, resulting in index out of bound error
            // Length(5),
            // Length(4),
            // Length(3),
            // Length(5),
            // Length(7),
            // Length(4),
            // Length(5),
            // Length(6),
        ];
        consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(111965),
            slots_per_epoch: Length(7140),
        };
        consensus_state.blockchain_length = Length(77748);
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state = consensus_state;
        chain_b.push(prot_state).unwrap();

        let mut chains = vec![];
        chains.push(chain_b);
        let result = chain_a.select_secure_chain(&chains);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &chain_a);
    }
}
