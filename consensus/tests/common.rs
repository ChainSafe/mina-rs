// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use hex::ToHex;
    use mina_consensus::common::*;
    use mina_crypto::prelude::*;
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
        assert_eq!(c.push(b1).unwrap_err(), ChainError::InvalidHeight,);
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
        assert_eq!(None, c.last_vrf());

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
        assert_eq!(expected, c.last_vrf());
    }
}
