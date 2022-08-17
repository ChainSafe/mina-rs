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
    use proof_systems::{mina_hasher::*, *};
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
        let roinput = staged_ledger_hash.to_chunked_roinput();
        assert_eq!(roinput, genesis_staged_ledger_hash_chunked_roinput()?);
        assert_eq!(
            ROInput::from(roinput),
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
            ROInput::from(body_reference.to_chunked_roinput()),
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
            ROInput::from(blockchain_state.to_chunked_roinput()),
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
