
use mina_serialization_types::staged_ledger_diff::{TransactionStatus, TransactionSnarkWork};
use test_fixtures::TEST_BLOCKS;


#[test]
fn test_deserialize_snark_work() {
	// Grab a block we know to contain snark work from the fixtures
	let block_fixture = TEST_BLOCKS.get("3NK9fHpzfPWhuxFhQ9Dau1X1JWtstB6kGC4xrurSPU1kctMCsU9U.hex").unwrap();

	// using the loosely deserialized form, extract the first snark work component
	// path: t/staged_ledger_diff/t/diff/t/0/t/t/completed_works/0
	let snark_work_example: &bin_prot::Value = &block_fixture.value["t"]["staged_ledger_diff"]["t"]["diff"]["t"][0]["t"]["t"]["completed_works"][0];
	println!("{:?}", snark_work_example);

	// we can reserialize the loosely deserialized version to get some bytes for the example snark work
	let mut snark_work_example_bytes = Vec::new();
	bin_prot::to_writer(&mut snark_work_example_bytes, snark_work_example).unwrap();
	// you could write these to a file and try to deserialize them directly into the TransactionSnarkWork type 
	// to make test turnaround time much quicker

	// Ideally we want this to pass
	// let snark_work: TransactionSnarkWorkV1 = bin_prot::from_reader(&snark_work_example_bytes[..]).unwrap();

	println!("{:?}", snark_work_example_bytes);
}
