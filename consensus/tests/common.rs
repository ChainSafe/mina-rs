// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use hex::ToHex;
    use mina_consensus::{common::*, density::ConsensusConstants, error::ConsensusError};
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
        let mut expected: ProtocolState = Default::default();
        expected.body.consensus_state.blockchain_length = Length(0);
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
        let constants = ConsensusConstants::from_genesis();
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
        let select_result = genesis_chain
            .select_secure_chain(&chains, &constants)
            .unwrap();
        let a = select_result.0.get(0).unwrap();
        assert_eq!(a.body.consensus_state.min_window_density, Length(43));
        assert_eq!(a.body.consensus_state.sub_window_densities, densities);
    }
}
