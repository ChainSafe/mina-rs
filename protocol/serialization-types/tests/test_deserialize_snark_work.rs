// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_serialization_types::v1::TransactionSnarkWorkV1;
use test_fixtures::TEST_BLOCKS_WITH_SNARK_WORK;

#[test]
fn test_deserialize_snark_work() {
    // Grab a block we know to contain snark work from the fixtures
    let block_fixture = TEST_BLOCKS_WITH_SNARK_WORK
        .get("3NK9fHpzfPWhuxFhQ9Dau1X1JWtstB6kGC4xrurSPU1kctMCsU9U.hex")
        .unwrap();

    // using the loosely deserialized form, extract the first snark work component
    // path: t/staged_ledger_diff/t/diff/t/0/t/t/completed_works/0
    let snark_work: &bin_prot::Value = &block_fixture.value["t"]["staged_ledger_diff"]["t"]["diff"]
        ["t"][0]["t"]["t"]["completed_works"][0];

    // Reserialize the loosely deserialized version to get some bytes for the snark work
    let mut snark_work_bytes = Vec::new();
    bin_prot::to_writer(&mut snark_work_bytes, snark_work).unwrap();

    let _snark_work: TransactionSnarkWorkV1 = bin_prot::from_reader(&snark_work_bytes[..]).unwrap();
}
