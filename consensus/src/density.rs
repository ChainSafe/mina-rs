// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! This module implements window density APIs to aid in long range fork
//! selection rule during chain selection in the mina consensus algorithm.

use crate::common::ConsensusConstants;
use crate::error::ConsensusError;
use mina_rs_base::consensus_state::ConsensusState;
use mina_rs_base::numbers::Length;

fn min(a: u32, b: u32) -> u32 {
    a.min(b)
}

fn max(a: u32, b: u32) -> u32 {
    a.max(b)
}

/// Computes the relative minimum window density of the given chains.
/// The minimum density value is used in the case of a long range fork
/// and the chain with the higher minimum window density is chosen as the canonical chain.
/// The need for relative density is explained here:
/// <https://github.com/MinaProtocol/mina/blob/02dfc3ff0160ba3c1bbc732baa07502fe4312b04/docs/specs/consensus/README.md#5412-relative-minimum-window-density>
pub fn relative_min_window_density(
    chain_a: &ConsensusState,
    chain_b: &ConsensusState,
    constants: &ConsensusConstants,
) -> Result<u32, ConsensusError> {
    let max_slot = max(
        chain_a.curr_global_slot.slot_number.0,
        chain_b.curr_global_slot.slot_number.0,
    );

    // grace-period rule
    if max_slot < constants.grace_period_end.0 {
        return Ok(chain_a.min_window_density.0);
    }

    let projected_window = {
        // compute shift count
        let mut shift_count = min(
            max(
                max_slot - chain_a.curr_global_slot.slot_number.0.saturating_sub(1),
                0,
            ),
            constants.sub_windows_per_window.0,
        );
        // initialize projected window based off of chain_a
        let mut projected_window = chain_a.sub_window_densities.clone();

        // relative sub window
        let mut rel_sub_window =
            chain_a.curr_global_slot.slot_number.0 % constants.sub_windows_per_window.0;

        // ring shift
        while shift_count > 0 {
            rel_sub_window = (rel_sub_window + 1) % constants.sub_windows_per_window.0;
            projected_window[rel_sub_window as usize] = Length(0);
            shift_count -= 1;
        }

        projected_window
    };

    // compute projected window density
    let projected_window_density = projected_window.iter().map(|s| s.0).sum();

    // compute minimum window density
    Ok(min(chain_a.min_window_density.0, projected_window_density))
}

#[cfg(test)]
mod tests {
    use mina_rs_base::{
        consensus_state::ConsensusState, global_slot::GlobalSlot, numbers::Length,
        types::GlobalSlotNumber,
    };
    use wasm_bindgen_test::*;

    use crate::common::ConsensusConstants;

    use super::relative_min_window_density;

    #[test]
    #[wasm_bindgen_test]
    fn genesis_to_block_at_height_2120_density() {
        // constants from genesis state
        let consensus_constants = ConsensusConstants::from_genesis();

        let mut chain_a = ConsensusState::default();
        chain_a.min_window_density = Length(77);
        chain_a.sub_window_densities = vec![
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

        chain_a.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(7140),
        };

        // block at 1500 height
        let mut chain_c = ConsensusState::default();

        chain_c.sub_window_densities = vec![
            Length(2),
            Length(7),
            Length(4),
            Length(5),
            Length(5),
            Length(4),
            Length(1),
            Length(5),
            Length(6),
            Length(3),
            Length(6),
        ];
        chain_c.min_window_density = Length(48);
        chain_c.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(2121),
            slots_per_epoch: Length(7140),
        };

        let a = relative_min_window_density(&chain_a, &chain_c, &consensus_constants);
        assert_eq!(a.unwrap(), 0);
        // comparing both of these the select_secure_chain ends up chossing 42 with chain_c as the canonical chain
        let a = relative_min_window_density(&chain_c, &chain_a, &consensus_constants);
        assert_eq!(a.unwrap(), 42);
    }

    #[test]
    #[wasm_bindgen_test]
    fn within_grace_period_picks_local_chain() {
        // genesis state
        let protocol_constants = ConsensusConstants::from_genesis();

        let mut genesis = ConsensusState::default();
        genesis.min_window_density = Length(77);
        genesis.sub_window_densities = vec![
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
        let genesis_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(0),
            slots_per_epoch: Length(7140),
        };

        genesis.curr_global_slot = genesis_global_slot;

        // block at 1000 height
        let mut chain_1000 = ConsensusState::default();

        chain_1000.sub_window_densities = vec![
            Length(6),
            Length(6),
            Length(2),
            Length(3),
            Length(5),
            Length(3),
            Length(5),
            Length(5),
            Length(5),
            Length(6),
            Length(7),
        ];
        chain_1000.min_window_density = Length(75); // originally 77, modified to 75 trigger grace rule
        chain_1000.curr_global_slot = GlobalSlot {
            slot_number: GlobalSlotNumber(1402),
            slots_per_epoch: Length(7140),
        };

        let a = relative_min_window_density(&genesis, &chain_1000, &protocol_constants);
        assert_eq!(a.unwrap(), 77);
    }
}
