// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {

    use bin_prot::BinProtRule;
    use bin_prot::{from_reader, to_writer, Deserializer, Value};
    use lazy_static::lazy_static;
    use mina_rs_base::numbers::{BlockTime, Delta, Length};
    use mina_rs_base::protocol_state::ProtocolConstants;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use mina_crypto::hash::StateHash;
    use mina_rs_base::{
        external_transition::ExternalTransition, protocol_version::ProtocolVersion,
    };

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
        static ref TEST_BLOCK_1: bin_prot::Value = load_test_block();
    }

    #[test]
    fn test_protocol_state_body_constants() {
        for block in [&TEST_BLOCK_1] {
            test_in_block::<Length>(block, &["t/protocol_state/t/t/body/t/t/constants/t/t/k"]);
            test_in_block::<Length>(
                block,
                &["t/protocol_state/t/t/body/t/t/constants/t/t/slots_per_epoch"],
            );
            test_in_block::<Length>(
                block,
                &["t/protocol_state/t/t/body/t/t/constants/t/t/slots_per_sub_window"],
            );
            test_in_block::<Delta>(
                block,
                &["t/protocol_state/t/t/body/t/t/constants/t/t/delta"],
            );
            test_in_block::<BlockTime>(
                block,
                &["t/protocol_state/t/t/body/t/t/constants/t/t/genesis_state_timestamp"],
            );
            test_in_block::<ProtocolConstants>(block, &["t/protocol_state/t/t/body/t/t/constants"]);
        }
    }

    #[test]
    fn test_all_block_subtypes() {
        ////////////////////////////////////////////////////////////////
        // Here is where to add calls to test_in_block for every type
        // that has a strongly typed implementation to test
        ////////////////////////////////////////////////////////////////

        for block in [&TEST_BLOCK_1] {
            // protocol_version
            test_in_block::<ProtocolVersion>(block, &["t/current_protocol_version"]);
            test_in_block::<Option<ProtocolVersion>>(block, &["t/proposed_protocol_version_opt"]);

            // state hash
            test_in_block::<StateHash>(block, &["t/protocol_state/t/t/previous_state_hash"]);
        }
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
        let block: &Value = &TEST_BLOCK_1;

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
        // check we can deserialize into this type without error
        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, &BLOCK_RULE);
        let block: ExternalTransition =
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

    fn load_test_block() -> bin_prot::Value {
        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, &BLOCK_RULE);
        Deserialize::deserialize(&mut de).expect("Failed to deserialize test block")
    }
}
