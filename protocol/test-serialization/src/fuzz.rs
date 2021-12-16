// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use crate::fuzz_test;
    use bin_prot::Deserializer;
    use mina_rs_base::types::*;
    use rand::prelude::*;
    use serde::Deserialize;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_corrupted_deserialization() {
        fuzz_test!(
            ExternalTransition

            ProtocolState
            ProtocolStateBody
            BlockchainState
            ConsensusState
            ProtocolConstants

            ProtocolStateProof
            ProofStatement
            PrevEvals
            Proof
            ProofMessages
            ProofOpenings

            StagedLedgerDiff
            StagedLedgerDiffTuple
            StagedLedgerPreDiffTwo

            DeltaTransitionChainProof

            Option<ProtocolVersion>
            ()
        );
    }

    #[macro_export]
    macro_rules! fuzz_test {
        ($($ty: ty) *) => {
            $(
                let mut rng = rand::thread_rng();
                for _i in 0..5 {
                    let mut bytes = vec![0; rng.gen_range((1024 * 1024)..(50 * 1024 * 1024))];
                    rng.try_fill_bytes(&mut bytes).unwrap();
                    let et: anyhow::Result<$ty> = (|| {
                        let mut de = Deserializer::from_reader(bytes.as_slice());
                        let et: $ty = Deserialize::deserialize(&mut de)?;
                        Ok(et)
                    })();
                    et.unwrap_err();
                }
            )*
        };
    }
}
