// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use anyhow::*;
    use ark_ff::BigInteger256;
    use mina_crypto::hash::*;
    use mina_hasher::Fp;
    use mina_rs_base::types::*;
    use num::BigUint;
    use proof_systems::{mina_hasher::*, mina_signer::CompressedPubKey, *};
    use std::str::FromStr;

    #[test]
    fn test_berkeley_genesis_block_chain_state_hash() -> Result<()> {
        let staged_ledger_hash = StagedLedgerHash {
            non_snark: NonSnarkStagedLedgerHash {
                ledger_hash: LedgerHash::from_str(
                    "jwxjg179rPZDX3N8x7rGs98NVKnXxQ6kauk4R421ZXEb551SPUu",
                )?,
                aux_hash: AuxHash::from_str("UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom")?,
                pending_coinbase_aux: PendingCoinbaseAuxHash::from_str(
                    "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
                )?,
            },
            pending_coinbase_hash: CoinBaseHash::from_str(
                "2n27mUhCEctJbiZQdrk3kxYc7DVHvJVDErjXrjNs7jnP3HMLKtuN",
            )?,
        };
        assert_eq!(
            staged_ledger_hash.to_chunked_roinput(),
            genesis_staged_ledger_hash_chunked_roinput()?
        );
        assert_eq!(
            staged_ledger_hash.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10(
                    "18312982411155638834795952767307088331002783393569971720271219236025400527059"
                )?)
                .append_field(fp_from_radix_10(
                    "3471521899433704832424762365110517199016372719455082716757770428876187682412"
                )?)
                .append_field(fp_from_radix_10("1")?)
        );

        let genesis_ledger_hash =
            LedgerHash::from_str("jwxjg179rPZDX3N8x7rGs98NVKnXxQ6kauk4R421ZXEb551SPUu")?;
        assert_eq!(
            genesis_ledger_hash.to_chunked_roinput(),
            ChunkedROInput::new().append_field(fp_from_radix_10(
                "13537175470369816875647086174838928722486573822187156126910528780791859041649"
            )?)
        );
        println!(
            "genesis_ledger_hash: {}",
            genesis_ledger_hash.to_chunked_roinput()
        );

        let local_state = BlockchainStateRegistersLocalState {
            stack_frame: Field::from_str_radix(
                "02F99BCFB0AA7F48C1888DA5A67196A2410FB084CD2DB1AF5216C5122AEBC054",
                16,
            )?,
            call_stack: Field::from_str_radix(
                "0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )?,
            transaction_commitment: Field::from_str_radix(
                "0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )?,
            full_transaction_commitment: Field::from_str_radix(
                "0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )?,
            token_id: TokenId(1),
            excess: SignedAmount(0, true),
            ledger: LedgerHash::from_str("jw6bz2wud1N6itRUHZ5ypo3267stk4UgzkiuWtAMPRZo9g4Udyd")?,
            success: true,
            party_index: MinaIndex(0),
            failure_status_tbl: Default::default(),
        };
        let roinput = local_state.to_chunked_roinput();
        assert_eq!(roinput, genesis_local_state_chunked_roinput()?);

        let registers = BlockchainStateRegisters {
            ledger: LedgerHash::from_str("jwxjg179rPZDX3N8x7rGs98NVKnXxQ6kauk4R421ZXEb551SPUu")?,
            pending_coinbase_stack: (),
            local_state,
        };
        let roinput = registers.to_chunked_roinput();
        assert_eq!(roinput, genesis_registers_chunked_roinput()?);

        let timestamp = BlockTime(1655755201000);
        assert_eq!(
            timestamp.to_chunked_roinput(),
            ChunkedROInput::new().append_packed(fp_from_radix_10("1655755201000")?, 64)
        );

        let body_reference = BodyReference::from_hex(
            "36bda176656cc3be96c3d317db7b4ac06fdbc7f4eedcd6efdd20e28143d67421",
        )?;
        assert_eq!(
            body_reference.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10(
                    "12296160664399627495704595388410991961901558829692462701693435696947689868193"
                )?)
                .append_field(fp_from_radix_10("0")?)
        );

        let blockchain_state = BlockchainState {
            staged_ledger_hash,
            genesis_ledger_hash,
            registers,
            timestamp,
            body_reference,
        };
        assert_eq!(
            blockchain_state.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10(
                    "18312982411155638834795952767307088331002783393569971720271219236025400527059"
                )?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "1345645986294164927562966675279626510497288257949713170124140298300287598676"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("1")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "3471521899433704832424762365110517199016372719455082716757770428876187682412"
                )?)
                .append_field(fp_from_radix_10(
                    "7237005577332262214169345992296663979832263477748145952816219112632247324463"
                )?)
                .append_field(fp_from_radix_10(
                    "59715292382037036695237625812635217064549677739652"
                )?)
        );

        // let consensus_state =
        //     protocol_state |> Mina_state.Protocol_state.consensus_state
        // in
        // let roinput =
        //     consensus_state |> Consensus.Data.Consensus_state.to_input_debug
        // in
        let consensus_state = ConsensusState {
            blockchain_length: 1.into(),
            epoch_count: 0.into(),
            min_window_density: 77.into(),
            sub_window_densities: vec![
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
            ],
            last_vrf_output: VrfOutputTruncated::from_str(
                "OruOTtGM3tJL3jM0GHtCzKyugvWT0ZP7VckspHX8_g8",
            )?,
            total_currency: 1013238001000001000.into(),
            curr_global_slot: GlobalSlot {
                slot_number: 0.into(),
                slots_per_epoch: 7140.into(),
            },
            global_slot_since_genesis: 0.into(),
            staking_epoch_data: EpochData {
                ledger: EpochLedger {
                    hash: LedgerHash::from_str(
                        "jwxjg179rPZDX3N8x7rGs98NVKnXxQ6kauk4R421ZXEb551SPUu",
                    )?,
                    total_currency: 1013238001000001000.into(),
                },
                seed: EpochSeed::from_str("2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA")?,
                start_checkpoint: StateHash::from_str(
                    "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                )?,
                lock_checkpoint: StateHash::from_str(
                    "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                )?,
                epoch_length: 1.into(),
            },
            next_epoch_data: EpochData {
                ledger: EpochLedger {
                    hash: LedgerHash::from_str(
                        "jwxjg179rPZDX3N8x7rGs98NVKnXxQ6kauk4R421ZXEb551SPUu",
                    )?,
                    total_currency: 1013238001000001000.into(),
                },
                seed: EpochSeed::from_str("2vc1zQHJx2xN72vaR4YDH31KwFSr5WHSEH2dzcfcq8jxBPcGiJJA")?,
                start_checkpoint: StateHash::from_str(
                    "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                )?,
                lock_checkpoint: StateHash::from_str(
                    "3NLi4a85TqcMwLAoezJjbBoYhS6x7EKyf5ThWhUS7NhDesqyXWbx",
                )?,
                epoch_length: 2.into(),
            },
            has_ancestor_in_same_checkpoint_window: true,
            block_stake_winner: CompressedPubKey::from_address(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )?,
            block_creator: CompressedPubKey::from_address(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )?,
            coinbase_receiver: CompressedPubKey::from_address(
                "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            )?,
            supercharge_coinbase: true,
        };
        assert_eq!(
            consensus_state.last_vrf_output.roinput(),
            ROInput::new().append_field(fp_from_radix_10(
                "5250504782050800269768049800766857209688182666876398046583012356259366563838"
            )?)
        );
        assert_eq!(
            consensus_state.curr_global_slot.to_chunked_roinput(),
            ChunkedROInput::new().append_u32(0).append_u32(7140)
        );
        assert_eq!(
            consensus_state.curr_global_slot.roinput(),
            ROInput::new().append_field(fp_from_radix_10("7140")?)
        );
        assert_eq!(
            consensus_state.staking_epoch_data.to_chunked_roinput(),
            ChunkedROInput::new()
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_u32(1)
                .append_u64(1013238001000001000)
        );
        assert_eq!(
            consensus_state.staking_epoch_data.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("19459982074709552616")?)
        );
        assert_eq!(
            consensus_state.next_epoch_data.to_chunked_roinput(),
            ChunkedROInput::new()
                .append_field(fp_from_radix_10(
                    "14681961814697422253233195325942500722138391379385252796689294365564545340151"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "9467349967580152589673091036870103925099662775818255640284311917171078832860"
                )?)
                .append_u32(2)
                .append_u64(1013238001000001000)
        );
        assert_eq!(
            consensus_state.next_epoch_data.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10(
                    "14681961814697422253233195325942500722138391379385252796689294365564545340151"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "9467349967580152589673091036870103925099662775818255640284311917171078832860"
                )?)
                .append_field(fp_from_radix_10("37906726148419104232")?)
        );

        assert_eq!(
            consensus_state.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "14681961814697422253233195325942500722138391379385252796689294365564545340151"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "9467349967580152589673091036870103925099662775818255640284311917171078832860"
                )?)
                .append_field(fp_from_radix_10(
                    "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                )?)
                .append_field(fp_from_radix_10(
                    "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                )?)
                .append_field(fp_from_radix_10(
                    "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                )?)
                .append_field(fp_from_radix_10(
                    "6277101735386680790037531676199156265425591762849762377735"
                )?)
                .append_field(fp_from_radix_10(
                    "47179906678498547680151595604921559418585159020371769110072453979228"
                )?)
                .append_field(fp_from_radix_10(
                    "8572332179547628723856916093832990528989133037876787922237980667902"
                )?)
                .append_field(fp_from_radix_10(
                    "1379148100938082216184032579547152646221531430054524878849"
                )?)
                .append_field(fp_from_radix_10(
                    "642215880070851285096273096370689904226955521856"
                )?)
        );

        // let constants =
        //     protocol_state |> Mina_state.Protocol_state.constants
        // in
        // let roinput = constants |> Protocol_constants_checked.to_input in
        let constants = ProtocolConstants {
            k: 290.into(),
            slots_per_epoch: 7140.into(),
            slots_per_sub_window: 7.into(),
            delta: 0.into(),
            genesis_state_timestamp: 1655755201000.into(),
        };
        assert_eq!(
            constants.to_chunked_roinput(),
            ChunkedROInput::new()
                .append_packed(fp_from_radix_10("290")?, 32)
                .append_packed(fp_from_radix_10("0")?, 32)
                .append_packed(fp_from_radix_10("7140")?, 32)
                .append_packed(fp_from_radix_10("7")?, 32)
                .append_packed(fp_from_radix_10("1655755201000")?, 64)
        );
        assert_eq!(
            constants.roinput(),
            ROInput::new().append_field(fp_from_radix_10(
                "423835474825961846844757681839698573328295964924392"
            )?)
        );

        let protocol_state_body = ProtocolStateBody {
            genesis_state_hash: StateHash::from_str(
                "3NLi4a85TqcMwLAoezJjbBoYhS6x7EKyf5ThWhUS7NhDesqyXWbx",
            )?,
            blockchain_state,
            consensus_state,
            constants,
        };
        assert_eq!(
            protocol_state_body.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10(
                    "9467349967580152589673091036870103925099662775818255640284311917171078832860"
                )?)
                .append_field(fp_from_radix_10(
                    "18312982411155638834795952767307088331002783393569971720271219236025400527059"
                )?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "1345645986294164927562966675279626510497288257949713170124140298300287598676"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("1")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "14681961814697422253233195325942500722138391379385252796689294365564545340151"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "13537175470369816875647086174838928722486573822187156126910528780791859041649"
                )?)
                .append_field(fp_from_radix_10(
                    "9467349967580152589673091036870103925099662775818255640284311917171078832860"
                )?)
                .append_field(fp_from_radix_10(
                    "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                )?)
                .append_field(fp_from_radix_10(
                    "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                )?)
                .append_field(fp_from_radix_10(
                    "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                )?)
                .append_field(fp_from_radix_10(
                    "1954596133368421387722139016445180426982017862454208282937095146054530"
                )?)
                .append_field(fp_from_radix_10(
                    "24187223653712748985369614349883567510519372552037957573041"
                )?)
                .append_field(fp_from_radix_10(
                    "784637717014678956011557744582442801400480448550706887870"
                )?)
                .append_field(fp_from_radix_10(
                    "238834422509354367104600567260149474239520236223349248839734070870016"
                )?)
                .append_field(fp_from_radix_10(
                    "483336833626235920455068665074195165254570252067109806800903"
                )?)
                .append_field(fp_from_radix_10(
                    "47179906678498547680151595604921559418592344102652004783299007279883"
                )?)
                .append_field(fp_from_radix_10(
                    "327365860291324522845756249582241551047679543985146121484579122664"
                )?)
                .append_field(fp_from_radix_10(
                    "41740486762171608190998085217754024788952913243275266"
                )?)
                .append_field(fp_from_radix_10("8105904008000008000")?)
        );

        Ok(())
    }

    // OCaml code snippet in mina_lib.ml
    // let genesis_block =
    //  Mina_block.genesis ~precomputed_values:config.precomputed_values
    // in
    // let genesis_block_with_hash, _genesis_block_validation =
    //   genesis_block
    // in
    // let genesis_block_data =
    //   genesis_block_with_hash |> With_hash.data
    // in
    // let genesis_block_hash =
    //   genesis_block_with_hash |> With_hash.hash
    // in
    // let staged_ledger_hash =
    //   genesis_block_data |> Mina_block.header
    //   |> Mina_block.Header.protocol_state
    //   |> Mina_state.Protocol_state.blockchain_state
    //   |> Mina_state.Blockchain_state.staged_ledger_hash
    // in
    // let staged_ledger_hash_roinput =
    //   staged_ledger_hash |> Staged_ledger_hash.to_input
    // in
    // Random_oracle_input.Chunked.print staged_ledger_hash_roinput
    //   Snark_params.Tick.Field.to_string ;
    // printf "packed:\"%d\",\n" (Array.length packed) ;
    //   for i = 0 to Array.length packed - 1 do
    //   printf "\t%s,\n" (packed.(i) |> Snark_params.Tick.Field.to_string)
    // done ;
    fn genesis_staged_ledger_hash_chunked_roinput() -> Result<ChunkedROInput> {
        let mut roi = ChunkedROInput::new().append_field(fp_from_radix_10(
            "18312982411155638834795952767307088331002783393569971720271219236025400527059",
        )?);
        for b in [
            0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1,
            1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0,
            1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1,
            1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0,
            0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1,
        ] {
            roi = roi.append_bool(b > 0);
        }
        Ok(roi)
    }

    // let registers =
    //   genesis_block_data |> Mina_block.header
    //   |> Mina_block.Header.protocol_state
    //   |> Mina_state.Protocol_state.blockchain_state
    //   |> Mina_state.Blockchain_state.registers
    // in
    // let local_state = registers |> Mina_state.Registers.local_state in
    // let roinput = local_state |> Mina_state.Local_state.to_input in
    // Random_oracle_input.Chunked.print roinput
    //   Pickles.Impls.Step.Field.Constant.to_string ;
    fn genesis_local_state_chunked_roinput() -> Result<ChunkedROInput> {
        Ok(ChunkedROInput::new()
            .append_field(fp_from_radix_10(
                "1345645986294164927562966675279626510497288257949713170124140298300287598676",
            )?)
            .append_field(fp_from_radix_10("0")?)
            .append_field(fp_from_radix_10("0")?)
            .append_field(fp_from_radix_10("0")?)
            .append_field(fp_from_radix_10("1")?)
            .append_field(fp_from_radix_10("0")?)
            .append_packed(fp_from_radix_10("0")?, 64)
            .append_packed(fp_from_radix_10("1")?, 1)
            .append_packed(fp_from_radix_10("0")?, 32)
            .append_packed(fp_from_radix_10("1")?, 1))
    }

    // let registers =
    //   genesis_block_data |> Mina_block.header
    //   |> Mina_block.Header.protocol_state
    //   |> Mina_state.Protocol_state.blockchain_state
    //   |> Mina_state.Blockchain_state.registers
    // in
    // let roinput = blockchain_state |> Mina_state.Blockchain_state.to_input_debug
    // Random_oracle_input.Chunked.print roinput
    //   Pickles.Impls.Step.Field.Constant.to_string ;
    //
    // Get result for certain field by commenting out fields in to_input_debug
    fn genesis_registers_chunked_roinput() -> Result<ChunkedROInput> {
        Ok(ChunkedROInput::new()
            .append_field(fp_from_radix_10(
                "13537175470369816875647086174838928722486573822187156126910528780791859041649",
            )?)
            .append_field(fp_from_radix_10(
                "1345645986294164927562966675279626510497288257949713170124140298300287598676",
            )?)
            .append_field(fp_from_radix_10("0")?)
            .append_field(fp_from_radix_10("0")?)
            .append_field(fp_from_radix_10("0")?)
            .append_field(fp_from_radix_10("1")?)
            .append_field(fp_from_radix_10("0")?)
            .append_packed(fp_from_radix_10("0")?, 64)
            .append_packed(fp_from_radix_10("1")?, 1)
            .append_packed(fp_from_radix_10("0")?, 32)
            .append_packed(fp_from_radix_10("1")?, 1))
    }

    fn fp_from_radix_10(s: &str) -> Result<Fp> {
        let big = BigUint::from_str(s)?;
        let big256: BigInteger256 = big.try_into().unwrap();
        Ok(big256.into())
    }
}
