// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0


#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use bin_prot::{to_writer, from_reader, Deserializer};

    const BLOCK_LAYOUT: &str = std::include_str!("../../layouts/external_transition.json");
    const BLOCK_BYTES: &[u8] = std::include_bytes!("../../test-fixtures/block");

    #[test]
    fn test_all_block_subtypes() {
        let mut deserializer = serde_json::Deserializer::from_str(BLOCK_LAYOUT);
        deserializer.disable_recursion_limit();
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
        let rule = bin_prot::Layout::deserialize(deserializer).unwrap().bin_prot_rule;

        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, rule);
        let block: bin_prot::Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize block");

        // Here is where to add calls to test_in_block for every type
        // that has a strongly typed implementation to test

        test_in_block::<mina_rs_base::protocol_version::ProtocolVersion>(&block, &["t/current_protocol_version"]);
        test_in_block::<Option<mina_rs_base::protocol_version::ProtocolVersion>>(&block, &["t/proposed_protocol_version_opt"]);
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
}
