// // Copyright 2020 ChainSafe Systems
// // SPDX-License-Identifier: Apache-2.0

// #[cfg(test)]
// mod tests {
//     use mina_consensus::common::{Chain, Consensus, ProtocolStateChain};
//     use mina_rs_base::{
//         consensus_state::ConsensusState,
//         global_slot::GlobalSlot,
//         numbers::Length,
//         types::{GlobalSlotNumber, ProtocolState},
//     };
//     use wasm_bindgen_test::*;

//     #[test]
//     #[wasm_bindgen_test]
//     fn genesis_to_block_at_height_2120_density() {
//         // constants from genesis state
//         let mut chain_a = ConsensusState::default();
//         chain_a.min_window_density = Length(77);
//         chain_a.sub_window_densities = vec![
//             Length(1),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//         ];

//         chain_a.curr_global_slot = GlobalSlot {
//             slot_number: GlobalSlotNumber(0),
//             slots_per_epoch: Length(7140),
//         };

//         let mut prot_state = ProtocolState::default();
//         prot_state.body.consensus_state = chain_a.clone();
//         let mut chain_a = ProtocolStateChain::default();
//         chain_a.push(prot_state).unwrap();

//         // block at 1500 height
//         let mut chain_c = ConsensusState::default();

//         chain_c.sub_window_densities = vec![
//             Length(2),
//             Length(7),
//             Length(4),
//             Length(5),
//             Length(5),
//             Length(4),
//             Length(1),
//             Length(5),
//             Length(6),
//             Length(3),
//             Length(6),
//         ];
//         chain_c.min_window_density = Length(48);
//         chain_c.curr_global_slot = GlobalSlot {
//             slot_number: GlobalSlotNumber(2121),
//             slots_per_epoch: Length(7140),
//         };

//         let mut prot_state = ProtocolState::default();
//         prot_state.body.consensus_state = chain_c.clone();
//         let mut chain_c = ProtocolStateChain::default();
//         chain_c.push(prot_state).unwrap();

//         let a = chain_a.relative_min_window_density(&chain_c).unwrap();

//         assert_eq!(a, 0);
//         // comparing both of these the select_secure_chain ends up chossing 47 with chain_c as the canonical chain
//         let c = chain_c.relative_min_window_density(&chain_a).unwrap();
//         assert_eq!(c, 47);
//     }

//     #[test]
//     #[wasm_bindgen_test]
//     fn within_grace_period_picks_local_chain() {
//         // genesis state
//         let mut genesis = ConsensusState::default();
//         genesis.min_window_density = Length(77);
//         genesis.sub_window_densities = vec![
//             Length(1),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//             Length(7),
//         ];
//         let genesis_global_slot = GlobalSlot {
//             slot_number: GlobalSlotNumber(0),
//             slots_per_epoch: Length(7140),
//         };

//         genesis.curr_global_slot = genesis_global_slot;

//         let mut genesis_state = ProtocolState::default();
//         genesis_state.body.consensus_state = genesis.clone();
//         let mut genesis = ProtocolStateChain::default();
//         genesis.push(genesis_state).unwrap();

//         // block at 1000 height
//         let mut chain_1000 = ConsensusState::default();

//         chain_1000.sub_window_densities = vec![
//             Length(6),
//             Length(6),
//             Length(2),
//             Length(3),
//             Length(5),
//             Length(3),
//             Length(5),
//             Length(5),
//             Length(5),
//             Length(6),
//             Length(7),
//         ];
//         chain_1000.min_window_density = Length(75); // originally 77, modified to 75 trigger grace rule
//         chain_1000.curr_global_slot = GlobalSlot {
//             slot_number: GlobalSlotNumber(1402),
//             slots_per_epoch: Length(7140),
//         };

//         let mut chain_1000_state = ProtocolState::default();
//         chain_1000_state.body.consensus_state = chain_1000.clone();
//         let mut chain_1000 = ProtocolStateChain::default();
//         chain_1000.push(chain_1000_state).unwrap();

//         let a = genesis.relative_min_window_density(&chain_1000);
//         assert_eq!(a.unwrap(), 77);
//     }
// }
