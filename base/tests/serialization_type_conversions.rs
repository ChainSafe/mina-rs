// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_rs_base::types::ExternalTransition;
use mina_serialization_types::v1::ExternalTransitionV1;
use test_fixtures::*;

#[test]
/// Test both From impls to convert between the serialization/wire-type of a block and the internal representation
/// The conversion must be lossless!
fn roundtrip_serialization_types() {
    let serialization_type_block: ExternalTransitionV1 = TEST_BLOCKS
        .get("block1")
        .expect("Failed to load block1")
        .external_transitionv1()
        .unwrap();
    let internal_type_block = ExternalTransition::from(serialization_type_block.clone());
    let recovered_serialization_type_block = ExternalTransitionV1::from(internal_type_block);

    assert_eq!(serialization_type_block, recovered_serialization_type_block);
}
