// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use anyhow::*;
    use ark_ff::BigInteger256;
    use mina_crypto::hash::*;
    use mina_hasher::Fp;
    use num::BigUint;
    use proof_systems::*;
    use std::str::FromStr;

    #[test]
    fn test_berkeley_genesis_block_chain_state_hash() -> Result<()> {
        let non_snark = NonSnarkStagedLedgerHash {
            ledger_hash: LedgerHash::from_str(
                "jwxjg179rPZDX3N8x7rGs98NVKnXxQ6kauk4R421ZXEb551SPUu",
            )?,
            aux_hash: AuxHash::from_str("UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom")?,
            pending_coinbase_aux: PendingCoinbaseAuxHash::from_str(
                "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
            )?,
        };
        let staged_ledger_hash = StagedLedgerHash {
            non_snark,
            pending_coinbase_hash: CoinBaseHash::from_str(
                "2n27mUhCEctJbiZQdrk3kxYc7DVHvJVDErjXrjNs7jnP3HMLKtuN",
            )?,
        };
        let roinput = staged_ledger_hash.to_chunked_roinput();
        assert_eq!(roinput, genesis_staged_ledger_hash_chunked_roinput()?);
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

    fn fp_from_radix_10(s: &str) -> Result<Fp> {
        let big = BigUint::from_str(s)?;
        let big256: BigInteger256 = big.try_into().unwrap();
        Ok(big256.into())
    }
}
