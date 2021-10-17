// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {

    use bin_prot::BinProtRule;
    use bin_prot::{from_reader, to_writer, Deserializer, Value};
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use wire_type::WireType;

    use mina_rs_base::protocol_version::ProtocolVersion;

    const BLOCK_LAYOUT: &str = std::include_str!("../../layouts/external_transition.json");
    const BLOCK_BYTES: &[u8] = std::include_bytes!("../../test-fixtures/block");

    // this allows the expensive block rule deserialization and parsing to be done only once for all tests
    lazy_static! {
        static ref BLOCK_RULE: BinProtRule = {
            let mut deserializer = serde_json::Deserializer::from_str(BLOCK_LAYOUT);
            deserializer.disable_recursion_limit();
            let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
            bin_prot::Layout::deserialize(deserializer)
                .unwrap()
                .bin_prot_rule
        };
    }

    #[test]
    fn test_all_block_subtypes() {
        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, &BLOCK_RULE);
        let block: bin_prot::Value =
            Deserialize::deserialize(&mut de).expect("Failed to deserialize block");

        ////////////////////////////////////////////////////////////////
        // Here is where to add calls to test_in_block for every type
        // that has a strongly typed implementation to test
        ////////////////////////////////////////////////////////////////

        // protocol_version
        test_in_block::<ProtocolVersion>(&block, &["t/current_protocol_version"]);
        test_in_block::<Option<ProtocolVersion>>(&block, &["t/proposed_protocol_version_opt"]);
    }

    fn test_in_block<'a, T: Serialize + Deserialize<'a>>(block: &bin_prot::Value, paths: &[&str]) {
        for path in paths {
            // pull out the bin_prot::Value corresponding to the path
            // will panic if the path is invalid
            let mut val = block;
            for p in path.split('/') {
                val = &val[p];
            }

            // write to binary then deserialize into T
            let mut bytes = vec![];
            bin_prot::to_writer(&mut bytes, val).expect("Failed writing bin-prot encoded data");
            let re_val: T = from_reader(bytes.as_slice()).expect("Could not deserialize type");

            // serialize back to binary and ensure it matches
            let mut re_bytes = vec![];
            to_writer(&mut re_bytes, &re_val).expect("Failed writing bin-prot encoded data");

            assert_eq!(bytes, re_bytes);
        }
    }

    #[test]
    fn smoke_test_roundtrip_block() {
        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, &BLOCK_RULE);
        let block: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize block");

        // test we can correctly index a known field
        assert_eq!(
            block["t"]["protocol_state"]["t"]["t"]["previous_state_hash"]["t"],
            Value::Tuple(
                vec![
                    30, 76, 197, 215, 115, 43, 42, 245, 198, 30, 253, 134, 49, 117, 82, 71, 182,
                    181, 180, 95, 18, 250, 46, 1, 25, 3, 78, 193, 57, 152, 116, 49,
                ]
                .iter()
                .map(|c| Value::Char(*c))
                .collect()
            )
        );

        // check roundtrip
        test_roundtrip(&block, &BLOCK_BYTES);
    }

    #[test]
    fn smoke_test_partial_block() {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
        #[serde(from = "<Self as WireType>::WireType")]
        #[serde(into = "<Self as WireType>::WireType")]
        struct PartialBlock {
            // unimplemented types
            protocol_state: Value,
            protocol_state_proof: Value,
            staged_ledger_diff: Value,
            delta_transition_chain_proof: Value,

            // implemented types
            current_protocol_version: ProtocolVersion,
            proposed_protocol_version_opt: Option<ProtocolVersion>,
            validation_callback: (),
        }

        // check we can deserialize into this type without error
        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, &BLOCK_RULE);
        let block: PartialBlock =
            Deserialize::deserialize(&mut de).expect("Failed to deserialize block");

        // check roundtrip
        test_roundtrip(&block, &BLOCK_BYTES);
    }

    fn test_roundtrip<T>(val: &T, bytes: &[u8])
    where
        T: Serialize,
    {
        let mut output = vec![];
        bin_prot::to_writer(&mut output, val).expect("Failed writing bin-prot encoded data");
        assert_eq!(bytes, output)
    }
}
