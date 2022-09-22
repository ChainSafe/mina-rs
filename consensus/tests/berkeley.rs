// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use anyhow::*;
    use mina_consensus::genesis::*;
    use mina_crypto::hash::*;
    use mina_rs_base::types::*;
    use proof_systems::{mina_hasher::*, *};

    #[test]
    fn test_berkeley_genesis_protocol_state_hash() -> Result<()> {
        let state = ProtocolState::berkeley();

        assert_eq!(
            state
                .body
                .blockchain_state
                .staged_ledger_hash
                .to_chunked_roinput(),
            genesis_staged_ledger_hash_chunked_roinput()?
        );

        println!(
            "genesis_ledger_hash: {}",
            state
                .body
                .blockchain_state
                .genesis_ledger_hash
                .to_chunked_roinput()
        );

        let roinput = state
            .body
            .blockchain_state
            .registers
            .local_state
            .to_chunked_roinput();
        assert_eq!(roinput, genesis_local_state_chunked_roinput()?);

        assert_eq!(
            state.body.blockchain_state.registers.to_chunked_roinput(),
            genesis_registers_chunked_roinput()?
        );

        assert_eq!(
            state.body.blockchain_state.timestamp.to_chunked_roinput(),
            ChunkedROInput::new().append_packed(fp_from_radix_10("1655755201000")?, 64)
        );

        assert_eq!(
            state.body.blockchain_state.body_reference.roinput(),
            ROInput::new()
                .append_field(fp_from_radix_10(
                    "12296160664399627495704595388410991961901558829692462701693435696947689868193"
                )?)
                .append_field(fp_from_radix_10("0")?)
        );

        // let consensus_state =
        //     protocol_state |> Mina_state.Protocol_state.consensus_state
        // in
        // let roinput =
        //     consensus_state |> Consensus.Data.Consensus_state.to_input_debug
        // in
        assert_eq!(
            state.body.consensus_state.last_vrf_output.roinput(),
            ROInput::new().append_field(fp_from_radix_10(
                "5250504782050800269768049800766857209688182666876398046583012356259366563838"
            )?)
        );
        assert_eq!(
            state
                .body
                .consensus_state
                .curr_global_slot
                .to_chunked_roinput(),
            ChunkedROInput::new().append_u32(0).append_u32(7140)
        );
        assert_eq!(
            state.body.consensus_state.curr_global_slot.roinput(),
            ROInput::new().append_field(fp_from_radix_10("7140")?)
        );
        assert_eq!(
            state
                .body
                .consensus_state
                .staking_epoch_data
                .to_chunked_roinput(),
            ChunkedROInput::new()
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "20038089104619582172254839672519820202817728273163650761198500757943363448868"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_u32(1)
                .append_u64(1013238001000001000)
        );
        assert_eq!(
            state
                .body
                .consensus_state
                .next_epoch_data
                .to_chunked_roinput(),
            ChunkedROInput::new()
                .append_field(fp_from_radix_10(
                    "14681961814697422253233195325942500722138391379385252796689294365564545340151"
                )?)
                .append_field(fp_from_radix_10("0")?)
                .append_field(fp_from_radix_10(
                    "20038089104619582172254839672519820202817728273163650761198500757943363448868"
                )?)
                .append_field(fp_from_radix_10(
                    "20469705092297587215111758345826930216920373655705023788336254043669748741566"
                )?)
                .append_u32(2)
                .append_u64(1013238001000001000)
        );

        // let constants =
        //     protocol_state |> Mina_state.Protocol_state.constants
        // in
        // let roinput = constants |> Protocol_constants_checked.to_input in
        assert_eq!(
            state.body.constants.to_chunked_roinput(),
            ChunkedROInput::new()
                .append_packed(fp_from_radix_10("290")?, 32)
                .append_packed(fp_from_radix_10("0")?, 32)
                .append_packed(fp_from_radix_10("7140")?, 32)
                .append_packed(fp_from_radix_10("7")?, 32)
                .append_packed(fp_from_radix_10("1655755201000")?, 64)
        );
        assert_eq!(
            state.body.constants.roinput(),
            ROInput::new().append_field(fp_from_radix_10(
                "423835474825961846844757681839698573328295964924392"
            )?)
        );

        // let protocol_state_body =
        //   protocol_state |> Mina_state.Protocol_state.body
        // in
        // printf "protocol_state_body hash: %s\n"
        //   ( protocol_state_body |> Mina_state.Protocol_state.Body.hash
        //   |> Snark_params.Tick.Field.to_string ) ;
        let state_body_hash = {
            let mut hasher = create_kimchi(());
            hasher.hash(&state.body)
        };
        assert_eq!(
            StateBodyHash::from(&state_body_hash).to_string(),
            "3WuwJxtzDb98vH8KLh7XtMAsoaskAo9bUCTiSM3EbWCjjhmtVwxe"
        );

        let state_hash = {
            let mut hasher = create_kimchi(());
            hasher.hash(state)
        };
        assert_eq!(
            StateHash::from(&state_hash).to_string(),
            "3NKrvXDzp7gskxqWUmwDJTFeSGA6ohYMjd38uKwDgkg8RH89QcgH"
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
    // printf "genesis_block_data:%s\n"
    //     (genesis_block_data |> Mina_block.to_yojson |> Yojson.Safe.to_string) ;
    // printf "genesis_block_hash:%s\n"
    // ( genesis_block_hash |> State_hash.State_hashes.to_yojson
    // |> Yojson.Safe.to_string ) ;
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
            1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1,
            1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1,
            0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1,
            0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0,
            0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1,
            0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1,
            1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1,
            0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0,
            1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0,
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
                "20038089104619582172254839672519820202817728273163650761198500757943363448868",
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
}
