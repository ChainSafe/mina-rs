// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use super::{block_path_test, block_path_test_batch};
    use bin_prot::BinProtRule;
    use bin_prot::{from_reader, to_writer, Deserializer, Value};
    use lazy_static::lazy_static;
    use mina_crypto::signature::PublicKey;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use mina_crypto::hash::*;
    use mina_rs_base::types::*;

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
    fn test_protocol_state_body() {
        block_path_test_batch! {
            ProtocolStateBody => "t/protocol_state/t/t/body"
        }
    }

    #[test]
    fn test_protocol_state_body_genesis_state_hash() {
        block_path_test_batch! {
            StateHash => "t/protocol_state/t/t/body/t/t/genesis_state_hash"
        }
    }

    #[test]
    fn test_protocol_state_body_blockchain_state() {
        block_path_test_batch! {
            SnarkedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/snarked_ledger_hash"
            SnarkedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/genesis_ledger_hash"
            TokenId => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/snarked_next_available_token"
            BlockTime => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/timestamp"
            BlockchainState => "t/protocol_state/t/t/body/t/t/blockchain_state"
        };
    }

    #[test]
    fn test_protocol_state_body_blockchain_state_staged_ledger_hash() {
        block_path_test_batch! {
            LedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark/t/ledger_hash"
            AuxHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark/t/aux_hash"
            AuxHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark/t/pending_coinbase_aux"
            NonSnarkStagedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark"
            CoinBaseHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/pending_coinbase_hash"
            StagedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash"
        };
    }

    #[test]
    fn test_protocol_state_body_consensus_state() {
        block_path_test_batch! {
            Length => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/blockchain_length"
            Length => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/epoch_count"
            Length => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/min_window_density"
            Vec<Length> => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/sub_window_densities"
            VrfOutputTruncated => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/last_vrf_output"
            Amount => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/total_currency"
            GlobalSlot => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/curr_global_slot"
            GlobalSlotNumber => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/global_slot_since_genesis"
            EpochData => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data"
            EpochData => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/next_epoch_data"
            bool => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/has_ancestor_in_same_checkpoint_window"
            PublicKey => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/block_stake_winner"
            ConsensusState => "t/protocol_state/t/t/body/t/t/consensus_state"
        }
    }

    #[test]
    fn test_protocol_state_body_consensus_state_staking_epoch_data() {
        block_path_test_batch! {
            EpochLedger => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data/t/t/ledger"
            EpochSeed => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data/t/t/seed"
            EpochData => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data"
        }
    }

    #[test]
    fn test_protocol_state_body_constants() {
        block_path_test_batch! {
            Length => "t/protocol_state/t/t/body/t/t/constants/t/t/k"
            Length => "t/protocol_state/t/t/body/t/t/constants/t/t/slots_per_epoch"
            Length => "t/protocol_state/t/t/body/t/t/constants/t/t/slots_per_sub_window"
            Delta => "t/protocol_state/t/t/body/t/t/constants/t/t/delta"
            BlockTime => "t/protocol_state/t/t/body/t/t/constants/t/t/genesis_state_timestamp"
            ProtocolConstants => "t/protocol_state/t/t/body/t/t/constants"
        }
    }

    #[test]
    fn test_all_block_subtypes() {
        ////////////////////////////////////////////////////////////////
        // Here is where to add calls to test_in_block for every type
        // that has a strongly typed implementation to test
        ////////////////////////////////////////////////////////////////
        block_path_test_batch! {
            ProtocolVersion => "t/current_protocol_version"
            Option<ProtocolVersion> => "t/proposed_protocol_version_opt"
            StateHash => "t/protocol_state/t/t/previous_state_hash"
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
            bin_prot::to_writer(&mut bytes, val).expect(&format!(
                "Failed writing bin-prot encoded data\npath: {}",
                path
            ));
            let re_val: T = from_reader(bytes.as_slice()).expect(&format!(
                "Could not deserialize type\npath: {}\nbytes({}): {:?}",
                path,
                bytes.len(),
                bytes
            ));

            // serialize back to binary and ensure it matches
            let mut re_bytes = vec![];
            to_writer(&mut re_bytes, &re_val).expect(&format!(
                "Failed writing bin-prot encoded data\npath: {}",
                path
            ));

            assert_eq!(bytes, re_bytes, "path: {}", path);
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

    #[macro_export]
    macro_rules! block_path_test {
        ($typ:ty, $path:expr) => {
            for block in [&TEST_BLOCK_1] {
                test_in_block::<$typ>(block, &[$path]);
            }
        };
    }

    #[macro_export]
    macro_rules! block_path_test_batch {
        ($($typ:ty => $path:expr) *)  => {
            $(
                block_path_test!($typ, $path);
            )*
        };
    }
}
