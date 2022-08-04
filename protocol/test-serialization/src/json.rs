// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use crate::*;
    use ark_ff::BigInteger256;
    use mina_rs_base::{types::*, *};
    use mina_serialization_types::json::*;
    use num::BigUint;
    use pretty_assertions::assert_eq;
    use proof_systems::mina_hasher::{create_legacy, Hashable, Hasher};
    use std::str::FromStr;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn json_binprot_parity_tests() {
        let block_fixture_binprot = test_fixtures::TEST_BLOCKS
            .get("3NKjZ5fjms6BMaH4aq7DopPGyMY7PbG6vhRsX5XnYRxih8i9G7dj.hex")
            .unwrap();
        let block_from_binprot =
            <ExternalTransition as BinProtSerializationType>::try_from_binprot(
                block_fixture_binprot.bytes.as_slice(),
            )
            .unwrap();
        let json_value = test_fixtures::JSON_TEST_BLOCKS
            .get("mainnet-117896-3NKjZ5fjms6BMaH4aq7DopPGyMY7PbG6vhRsX5XnYRxih8i9G7dj.json")
            .unwrap();
        let block_json: <ExternalTransition as JsonSerializationType>::T =
            serde_json::from_value(json_value.clone()).unwrap();
        let block_from_json = block_json.into();
        assert_eq!(block_from_binprot, block_from_json);
    }

    /*
    Parity test code:

    let%test_unit "test_genesis_protocol_state_to_random_oracle_input" =
    let open Core_kernel in
    let open Async in
    let json =
      In_channel.with_file "/path-to/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json" ~f:(fun in_channel ->
          try Yojson.Safe.from_channel in_channel with
          | _ ->
              Core.exit 1)
    in
    let precomputed =
      match Mina_block.Precomputed.of_yojson json with
      | Ok precomputed ->
          precomputed
      | Error _ ->
          Core.exit 1
    in
    let protocal_state = precomputed.protocol_state in
    let _ = protocal_state in
    let state_hash = Mina_state.Protocol_state.hashes_abstract ~hash_body: Mina_state.Protocol_state.Body.hash protocal_state in
    printf "state_hash:%s\n" (state_hash.state_hash |> State_hash.to_yojson |> Yojson.Safe.to_string);
    printf "state_hash:%s\n" (state_hash.state_hash |> State_hash.to_decimal_string);
    let body_hash = Mina_state.Protocol_state.Body.hash protocal_state.body in
    printf "body_hash:%s\n" (body_hash |> Mina_base.State_body_hash.to_yojson |> Yojson.Safe.to_string);
    printf "body_hash:%s\n" (body_hash |> Mina_base.State_body_hash.to_decimal_string);
    let input_to_string i = i |> Random_oracle.pack_input |> Random_oracle.hash |> State_hash.of_hash |> State_hash.to_decimal_string in
    let body_constants = Mina_state.Protocol_state.Body.constants protocal_state.body in
    printf "body_constants_hash:%s\n" (body_constants |> Protocol_constants_checked.to_input |> input_to_string);
    let consensus_state = Mina_state.Protocol_state.Body.consensus_state protocal_state.body in
    printf "consensus_state_hash:%s\n" (consensus_state |> Consensus.Data.Consensus_state.to_input |> input_to_string);
    let blockchain_state = Mina_state.Protocol_state.Body.blockchain_state protocal_state.body in
    printf "blockchain_state_hash:%s\n" (blockchain_state |> Mina_state.Blockchain_state.to_input |> input_to_string);
    let snarked_next_available_token = blockchain_state.snarked_next_available_token in
    printf "snarked_next_available_token_hash:%s\n" (snarked_next_available_token |> Token_id.to_input |> input_to_string);
    let timestamp = blockchain_state.timestamp in
    printf "timestamp_hash:%s\n" (timestamp |> Block_time.Bits.to_bits |> Random_oracle.Input.bitstring |> input_to_string);
    let staged_ledger_hash = blockchain_state.staged_ledger_hash in
    printf "staged_ledger_hash_hash:%s\n" (staged_ledger_hash |> Staged_ledger_hash.to_input |> input_to_string);
    let non_snark = blockchain_state.staged_ledger_hash |> Staged_ledger_hash.non_snark in
    printf "non_snark_digest:%s\n" (non_snark |> Staged_ledger_hash.Non_snark.digest |> Hex.encode);
    printf "non_snark_hash:%s\n" (non_snark |> Staged_ledger_hash.Non_snark.to_input |> input_to_string);
    printf "\n"

    Output:
    state_hash:"3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS"
    state_hash:18109765379584684499155740919947103416101561945742376017305891046236717214321
    body_hash:"3Wu8htSn6KNAFLGrrKiFFb2J6FzdnL14GCgNXP1KGdv8UZa3u1Qq"
    body_hash:11547288559214200277520549031042137594317244691846831172842173442778999413309
    body_constants_hash:21283551842411620881532468880644102678972422874752466180084533585306358443047
    consensus_state_hash:4567520866406870569587277062959652446267293244931899281786637782228092413468
    blockchain_state_hash:22984217591685958776680303176697091613340796570000046393879843972385768898212
    snarked_next_available_token_hash:5944847923306304475446774703885080487726156474234720095593426533796586636239
    timestamp_hash:48646600159211660110710851180350202765276633001045178862203248356708033087
    staged_ledger_hash_hash:8746300742690142157461847113315645738247638524643087279433100997652654346784
    non_snark_digest:1f3f4109e8e81048dfa42023b6292e8cc0c54721a34097e4333c0c89cefc9c26
    non_snark_hash:26916965920202625828811831006264431911594993787410334374666335644820635123853
    */
    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_hash_parity_test() {
        let json_value = test_fixtures::JSON_TEST_BLOCKS
            .get("mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json")
            .unwrap();
        let block_json: <ExternalTransition as JsonSerializationType>::T =
            serde_json::from_value(json_value.clone()).unwrap();
        let block: ExternalTransition = block_json.into();

        fn hash<T: Hashable<D = ()>>(t: &T) -> String {
            let mut hasher = create_legacy(());
            let hash = hasher.hash(t);
            let big256: BigInteger256 = hash.into();
            let big: BigUint = big256.into();
            big.to_str_radix(10)
        }

        assert_eq!(
            hash(&block.protocol_state.body.constants),
            "21283551842411620881532468880644102678972422874752466180084533585306358443047"
        );

        assert_eq!(
            hash(&block.protocol_state.body.consensus_state.last_vrf_output),
            "25370206210950520779082257948618326030877049579889865033411963068050002039007"
        );

        assert_eq!(
            hash(&block.protocol_state.body.consensus_state),
            "4567520866406870569587277062959652446267293244931899281786637782228092413468"
        );

        assert_eq!(
            hash(&block.protocol_state.body.blockchain_state),
            "22984217591685958776680303176697091613340796570000046393879843972385768898212"
        );

        assert_eq!(
            hash(
                &block
                    .protocol_state
                    .body
                    .blockchain_state
                    .snarked_next_available_token
            ),
            "5944847923306304475446774703885080487726156474234720095593426533796586636239"
        );

        assert_eq!(
            hash(
                &block
                    .protocol_state
                    .body
                    .blockchain_state
                    .snarked_next_available_token
            ),
            "5944847923306304475446774703885080487726156474234720095593426533796586636239"
        );

        assert_eq!(
            hash(&block.protocol_state.body.blockchain_state.timestamp),
            "48646600159211660110710851180350202765276633001045178862203248356708033087"
        );

        assert_eq!(
            hash(
                &block
                    .protocol_state
                    .body
                    .blockchain_state
                    .staged_ledger_hash
            ),
            "8746300742690142157461847113315645738247638524643087279433100997652654346784"
        );

        assert_eq!(
            hex::encode(
                block
                    .protocol_state
                    .body
                    .blockchain_state
                    .staged_ledger_hash
                    .non_snark
                    .digest()
            ),
            "1f3f4109e8e81048dfa42023b6292e8cc0c54721a34097e4333c0c89cefc9c26"
        );

        assert_eq!(
            hash(
                &block
                    .protocol_state
                    .body
                    .blockchain_state
                    .staged_ledger_hash
                    .non_snark
            ),
            "26916965920202625828811831006264431911594993787410334374666335644820635123853"
        );

        assert_eq!(
            hash(&block.protocol_state.body),
            "11547288559214200277520549031042137594317244691846831172842173442778999413309"
        );

        assert_eq!(
            hash(&block.protocol_state),
            "18109765379584684499155740919947103416101561945742376017305891046236717214321"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn json_binprot_state_hash_tests() {
        for (file_name, json) in test_fixtures::JSON_TEST_BLOCKS.iter() {
            let expected_state_hash = file_name
                .split('.')
                .nth(0)
                .unwrap_or_default()
                .split('-')
                .last()
                .unwrap_or_default();
            let block_json: <ExternalTransition as JsonSerializationType>::T =
                serde_json::from_value(json.clone()).unwrap();
            let block: ExternalTransition = block_json.into();
            let state_hash = block.protocol_state.state_hash();
            assert_eq!(&state_hash.to_string(), expected_state_hash);
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn consensus_state_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ConsensusState,
            ConsensusStateJson,
            "protocol_state/body/consensus_state"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn constants_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolConstants,
            ProtocolConstantsJson,
            "protocol_state/body/constants"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn blockchain_state_json_serde_roundtrip() {
        json_serde_roundtrip!(
            BlockchainStateLegacy,
            BlockchainStateJson,
            "protocol_state/body/blockchain_state"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_body_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolStateBody,
            ProtocolStateBodyJson,
            "protocol_state/body"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_json_serde_roundtrip() {
        json_serde_roundtrip!(ProtocolState, ProtocolStateJson, "protocol_state");
    }

    #[test]
    #[wasm_bindgen_test]
    fn protocol_state_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolStateProof,
            ProtocolStateProofBase64Json,
            "protocol_state_proof"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn delta_transition_chain_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(
            DeltaTransitionChainProof,
            DeltaTransitionChainProofJson,
            "delta_transition_chain_proof"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_json_serde_roundtrip() {
        json_serde_roundtrip!(
            TransactionSnarkWork,
            TransactionSnarkWorkJson,
            "staged_ledger_diff/diff/0/completed_works/0"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_statement_json_serde_roundtrip() {
        json_serde_roundtrip!(
            Statement,
            StatementJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/statement"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProtocolStateProof,
            ProtocolStateProofJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn completed_works_proofs_proof_statement_json_serde_roundtrip() {
        json_serde_roundtrip!(
            ProofStatement,
            ProofStatementJson,
            "staged_ledger_diff/diff/0/completed_works/0/proofs/1/proof/statement"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn commands_json_serde_roundtrip() {
        json_serde_roundtrip!(
            UserCommandWithStatus,
            UserCommandWithStatusJson,
            "staged_ledger_diff/diff/0/commands/0"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn user_command_json_serde_roundtrip() {
        json_serde_roundtrip!(
            UserCommand,
            UserCommandJson,
            "staged_ledger_diff/diff/0/commands/0/data"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn user_command_json_serde_roundtrip_2() {
        json_serde_roundtrip!(
            UserCommand,
            UserCommandJson,
            "staged_ledger_diff/diff/0/commands/2/data"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn transaction_status_json_serde_roundtrip() {
        json_serde_roundtrip!(
            TransactionStatus,
            TransactionStatusJson,
            "staged_ledger_diff/diff/0/commands/0/status"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn internal_command_balances_json_serde_roundtrip() {
        json_serde_roundtrip!(
            InternalCommandBalanceData,
            InternalCommandBalanceDataJson,
            "staged_ledger_diff/diff/0/internal_command_balances/0"
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn coinbase_json_serde_roundtrip() {
        json_serde_roundtrip!(CoinBase, CoinBaseJson, "staged_ledger_diff/diff/0/coinbase");
    }

    #[test]
    #[wasm_bindgen_test]
    fn staged_ledger_diff_json_serde_roundtrip() {
        json_serde_roundtrip!(StagedLedgerDiff, StagedLedgerDiffJson, "staged_ledger_diff");
    }

    #[test]
    #[wasm_bindgen_test]
    fn block_json_serde_roundtrip() {
        json_serde_roundtrip!(ExternalTransition, ExternalTransitionJson, "");
    }

    #[macro_export]
    macro_rules! json_serde_roundtrip {
        ($ty: ty, $ty_json: ty, $path: literal) => {
            (|| {
                'outer: for (_, mut json) in test_fixtures::JSON_TEST_BLOCKS.iter() {
                    if $path.len() > 0 {
                        for p in $path.split('/') {
                            json = match usize::from_str(p) {
                                Ok(index) => {
                                    if let Some(array) = json.as_array() {
                                        if index >= array.len() {
                                            continue 'outer;
                                        }
                                        &array[index]
                                    } else {
                                        panic!("Array expect");
                                    }
                                }
                                _ => &json[p],
                            };
                        }
                    }
                    let cs: $ty = {
                        let json_string = serde_json::to_string_pretty(json)?;
                        let json: $ty_json = serde_json::from_str(json_string.as_str())
                            .map_err(|err| anyhow::Error::msg(format!("{json_string}\n\n{err}")))?;
                        json.into()
                    };
                    let json_from_cs = {
                        let json: $ty_json = cs.into();
                        serde_json::to_value(&json)?
                    };
                    assert_eq!(json, &json_from_cs);
                }
                Ok::<_, anyhow::Error>(())
            })()
            .unwrap();
        };
    }
}
