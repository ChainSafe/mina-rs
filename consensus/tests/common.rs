// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_consensus::{common::*, error::ConsensusError};
    use mina_rs_base::types::ExternalTransition;
    use mina_rs_base::{types::*, JsonSerializationType};
    use std::str::FromStr;
    use wasm_bindgen_test::*;

    // Test Block Addition to ProtocolStateChain
    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_push() {
        // Init empty chain
        let mut test_chain: ProtocolStateChain = ProtocolStateChain(vec![]);
        assert_eq!(test_chain.length(), 0);

        // Case 1: Add Block with ValidHeight
        // Init new block `b0`, add to test_chain
        // Increases blockchain length by 1
        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        test_chain.push(b0).unwrap();
        assert_eq!(test_chain.length(), 1);
        // Init new block `b1`, add to test_chain
        // Increases blockchain length by 1
        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        test_chain.push(b1).unwrap();
        assert_eq!(test_chain.length(), 2);
        // Init new block `b2`, add to test_chain
        // Increases blockchain length by 1
        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(2);
        test_chain.push(b2).unwrap();
        assert_eq!(test_chain.length(), 3);

        // Case 2: Should fail to add Block with InvalidHeight
        // Init new block `b3`, with InvalidHeight
        // No change in blockchain length
        let mut b3: ProtocolState = Default::default();
        b3.body.consensus_state.blockchain_length = Length(1);
        assert_eq!(
            test_chain.push(b3).unwrap_err(),
            ConsensusError::InvalidHeight
        );
        assert_eq!(test_chain.length(), 3);
    }

    // Test ProtocolStateChain top() method
    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_top() {
        let mut test_chain: ProtocolStateChain = ProtocolStateChain(vec![]);
        // Case 1: Empty chain, top -> None
        assert_eq!(test_chain.length(), 0);
        assert_eq!(test_chain.top(), None);

        // Case 2: NonEmpty chain
        // Add new block `b0`, top -> b0
        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        test_chain.push(b0.clone()).unwrap();
        assert_eq!(test_chain.length(), 1);
        assert_eq!(test_chain.top(), Some(&b0)); // b0 is the latest added block

        // Add new block `b1`, top -> b1
        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        test_chain.push(b1.clone()).unwrap();
        assert_eq!(test_chain.length(), 2);
        assert_eq!(test_chain.top(), Some(&b1)); // b1 is the latest added block
    }

    // Test ProtocolStateChain epoch_slot() methods
    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_epoch_slot() {
        let mut test_chain: ProtocolStateChain = ProtocolStateChain(vec![]);
        // Case 1: GlobalSlot slot_number lesser than slots_per_epoch
        // Add new block `b0` with mocked data
        let mut b0: ProtocolState = Default::default();
        b0.body.consensus_state.blockchain_length = Length(0);
        b0.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(1000),
        };
        test_chain.push(b0).unwrap();
        let epoch_slot = test_chain.epoch_slot();
        assert_eq!(epoch_slot, Some(0)); // slot_number(GlobalSlotNumber(0)) % slots_per_epoch(Length(1000))

        // Add new block `b1` with mocked data
        let mut b1: ProtocolState = Default::default();
        b1.body.consensus_state.blockchain_length = Length(1);
        b1.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1),
            slots_per_epoch: Length(1000),
        };
        test_chain.push(b1).unwrap();
        let epoch_slot = test_chain.epoch_slot();
        assert_eq!(epoch_slot, Some(1)); // slot_number(GlobalSlotNumber(1)) % slots_per_epoch(Length(1000))

        // Case 2: GlobalSlot slot_number greater than slots_per_epoch
        // Add new block `b2` with mocked data
        let mut b2: ProtocolState = Default::default();
        b2.body.consensus_state.blockchain_length = Length(2);
        b2.body.consensus_state.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1002),
            slots_per_epoch: Length(1000),
        };
        test_chain.push(b2).unwrap();
        let epoch_slot = test_chain.epoch_slot();
        assert_eq!(epoch_slot, Some(2)); // slot_number(GlobalSlotNumber(1002)) % slots_per_epoch(Length(1000))
    }

    // Same BlockChain length and equal last vrf output of candidate chain but different state hash
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-117896-3NLPBDTckSdjcUFcQiE9raJsyzB84KayMPKi4PmwNybnA6J75GoL.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-117896-3NKrv92FYZFHRNUJxiP7VGeRx3MeDY2iffFjUWXTPoXJorsS63ba.json
    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_chain_state_hash() {
        // Init current chain
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-117896-3NLPBDTckSdjcUFcQiE9raJsyzB84KayMPKi4PmwNybnA6J75GoL.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-117896-3NKrv92FYZFHRNUJxiP7VGeRx3MeDY2iffFjUWXTPoXJorsS63ba.json",
        );
        let mut candidate_chain = ProtocolStateChain::default();
        candidate_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        let select_result = current_chain.select_longer_chain(&candidate_chain).unwrap(); // Candidate chain has greater state hash
        assert_eq!(select_result, &candidate_chain);
    }

    // Test longer chain selection method
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-117896-3NKrv92FYZFHRNUJxiP7VGeRx3MeDY2iffFjUWXTPoXJorsS63ba.json
    #[test]
    #[wasm_bindgen_test]
    fn selects_longer_chain() {
        // Init current chain
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-117896-3NKrv92FYZFHRNUJxiP7VGeRx3MeDY2iffFjUWXTPoXJorsS63ba.json",
        );
        let mut candidate_chain = ProtocolStateChain::default();
        candidate_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        let select_result = current_chain.select_longer_chain(&candidate_chain).unwrap(); // Candidate chain is longer 117896 > 77748
        assert_eq!(select_result, &candidate_chain);
    }

    #[test]
    #[wasm_bindgen_test]
    // Test select longer chain selection method last vrf cmp tie breaker logic
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    fn test_longer_chain_with_same_chain_length_greater_last_vrf_output() {
        // Init current chain
        // current chain vrf hex: "dd55ef09c0474817a64efffa7fe5a3aedd2db04a5f66e52e9630b59711f56613"
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        // candidate chain vrf hash hex: "e907e63d043c78b3dfa724b2ddc1152114fc91b983b40581b1036a8d19eb136d"
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut candidate_chain = ProtocolStateChain::default();
        candidate_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        // Candidate chain has greater last vrf hash
        let select_result = current_chain.select_longer_chain(&candidate_chain).unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(select_result, &candidate_chain);
        assert_eq!(
            result_state.body.consensus_state.last_vrf_output,
            VrfOutputTruncated::from_str("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=").unwrap() // last vrf output of candidate chain
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Test select longer chain selection method last vrf cmp tie breaker logic
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json
    fn test_longer_chain_with_same_chain_length_lesser_last_vrf_output() {
        // Init current chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json",
        );
        let mut candidate_chain = ProtocolStateChain::default();
        candidate_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        // Current chain has greater last vrf hash
        let select_result = current_chain.select_longer_chain(&candidate_chain).unwrap(); // Current chain has greater last vrf output
        assert_eq!(select_result, &current_chain);
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.last_vrf_output,
            VrfOutputTruncated::from_str("kKr83LYd7DyFupRAPh5Dh9eWM1teSEs5VjU4XId2DgA=").unwrap() // last vrf output of candidate chain
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Test select secure chain method
    // Current chain and candidate chain satisfy short range fork condition and current chain is longer than candidate chain
    fn test_select_secure_chain_short_range_fork() {
        // Init current chain with mocked data
        let mut current_chain = ProtocolStateChain::default();
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state.epoch_count = 5.into();
        prot_state.body.consensus_state.blockchain_length = 11.into();
        current_chain.push(prot_state).unwrap();

        // Init new chain with mocked data to satisfy short range fork rule wrt current chain
        let mut new_chain = ProtocolStateChain::default();
        let mut prot_state = ProtocolState::default();
        prot_state.body.consensus_state.epoch_count = 5.into();
        prot_state.body.consensus_state.blockchain_length = 10.into();
        new_chain.push(prot_state).unwrap();

        // Add new chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(new_chain);
        // Current chain and candidate chain satisfy short range fork condition and current chain is longer than candidate chain
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        assert_eq!(select_result, &current_chain);
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(11)
        );
        assert_eq!(result_state.body.consensus_state.epoch_count, Length(5));
    }

    #[test]
    #[wasm_bindgen_test]
    // Test select secure chain method long range fork with candidate chain having greater relative min window density
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    fn test_select_secure_chain_long_range_fork_greater_relative_min_window_density() {
        // Init current chain with mocked data

        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json",
        );

        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init new candidate chain with mocked data to satisfy long range fork rule wrt to current chain
        // new candidate chain has greater relative min window density

        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut candidate_chain = ProtocolStateChain::default();
        candidate_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        // Add new chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(candidate_chain);

        // Current chain and candidate chain satisfy long range fork rule and candidate chain has greater relative min window density
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        assert_eq!(select_result, &candidate_chains[0]);
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Test select secure chain method long range fork tie breaker logic
    // Current chain and candidate chain are long range fork, with same relative min window density
    fn test_select_secure_chain_long_range_fork_same_relative_min_window_density() {
        // Init current chain with mocked data to satisfy long rang fork condition
        let mut current_chain = ProtocolStateChain::default();
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
        current_chain.push(prot_state).unwrap();

        // Init new candidate chain with mocked data to satisfy long range fork rule wrt to current chain
        // new chain is the longer than current chain
        let mut new_chain = ProtocolStateChain::default();
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
        new_chain.push(prot_state).unwrap();

        // Add new chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(new_chain);
        // Current chain and candidate chain are long range fork, with same relative min window density
        // and candidate chain is longer wrt to current chain
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(select_result, &candidate_chains[0]);
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(77748)
        );
        // Verify select secure chain and select longer chain methods return same result
        // for chains which satisfies long range fork rule and have same relative min window density
        assert_eq!(
            result_state,
            current_chain
                .select_longer_chain(&candidate_chains[0])
                .unwrap()
                .0
                .get(0)
                .unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Test select secure chain satisfying long range fork with candidate chain having lesser relative min window density
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    fn test_select_secure_chain_long_range_fork_lesser_relative_min_window_density() {
        // Init current chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json",
        );
        let mut new_chain = ProtocolStateChain::default();
        new_chain.push(block_from_json.protocol_state).unwrap();

        // Add new chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(new_chain);
        // Current chain and candidate chain are long range fork and candidate chain has lesser relative min window density)
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        assert_eq!(select_result, &current_chain);
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Adversary Case: Candidate chain with extra sub window densities to result in higher relative min window density
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    fn adversary_test_select_secure_chain_candidate_with_extra_sub_window_densities() {
        // Init current chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let mut block_from_json: ExternalTransition = read_block_json(
            "mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json",
        );
        // Extra Sub Window Density added which could result in greater relative min window density and incorrect chain selection
        block_from_json
            .protocol_state
            .body
            .consensus_state
            .sub_window_densities
            .extend_from_slice(&[Length(6), Length(6), Length(6)]);
        let mut adversary_chain = ProtocolStateChain::default();
        adversary_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        // Add adversary chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(adversary_chain);
        // select secure chain method checks for extra sub window density in candidate chain and discard it to select valid chain
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        assert_eq!(select_result, &current_chain);
        let result_state = select_result.0.get(0).unwrap();
        assert_eq!(
            result_state.body.consensus_state.blockchain_length,
            Length(113267)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    // Adversary Case: Candidate chain with extremely high sub window densities to result in higher relative min window density
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    fn adversary_test_select_secure_chain_with_extremely_high_sub_window_densities() {
        // Init current chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let mut block_from_json: ExternalTransition = read_block_json(
            "mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json",
        );
        // changed window density of 3 to 999: An impossibly high sub window density
        // which will result in greater relative min window density and could lead to incorrect chain selection
        block_from_json
            .protocol_state
            .body
            .consensus_state
            .sub_window_densities[5] = Length(999);
        let mut adversary_chain = ProtocolStateChain::default();
        adversary_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        // Add adversary chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(adversary_chain);
        // select secure chain method checks for extremely high sub window density and discard it to select valid chain
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        assert_eq!(select_result, &current_chain);
    }

    #[test]
    #[wasm_bindgen_test]
    // Adversary Case: Candidate chain having very less number of sub window densities
    // Current chain: https://storage.googleapis.com/mina_network_block_data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json
    // Candidate chains: https://storage.googleapis.com/mina_network_block_data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json
    fn adversary_test_select_secure_chain_with_less_number_of_sub_window_densities() {
        // Init current chain from JSON
        let block_from_json: ExternalTransition = read_block_json(
            "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json",
        );
        let mut current_chain = ProtocolStateChain::default();
        current_chain.push(block_from_json.protocol_state).unwrap();

        // Init candidate chain from JSON
        let mut block_from_json: ExternalTransition = read_block_json(
            "mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json",
        );
        // Below sub window densities are delibrately removed
        block_from_json
            .protocol_state
            .body
            .consensus_state
            .sub_window_densities = vec![Length(6), Length(1), Length(3)];
        let mut adversary_chain = ProtocolStateChain::default();
        adversary_chain
            .push(block_from_json.protocol_state)
            .unwrap();

        // Add adversary chain to collection of candidate chains
        let mut candidate_chains = vec![];
        candidate_chains.push(adversary_chain);
        let select_result = current_chain
            .select_secure_chain(&candidate_chains)
            .unwrap();
        // select secure chain picks up the chain with valid sub window density
        assert_eq!(select_result, &current_chain);
    }

    // block path: mainnet-$BlockHeight-$StateHash.json eg: "mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json"
    // Note: block path must exist in test_fixtures::JSON_TEST_BLOCKS
    fn read_block_json(block_path: &str) -> ExternalTransition {
        let json_block = test_fixtures::JSON_TEST_BLOCKS.get(block_path).unwrap();
        let json_value: <ExternalTransition as JsonSerializationType>::T =
            serde_json::from_value(json_block.clone()).unwrap();
        json_value.into()
    }
}
