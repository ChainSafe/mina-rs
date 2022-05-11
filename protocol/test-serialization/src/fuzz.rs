// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use crate::fuzz_test;
    use bin_prot::Deserializer;
    use mina_serialization_types::v1::*;
    use rand::prelude::*;
    use serde::Deserialize;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_corrupted_deserialization() {
        fuzz_test!(
            ExternalTransitionV1

            ProtocolStateV1
            ProtocolStateBodyV1
            BlockchainStateV1
            ConsensusStateV1
            ProtocolConstantsV1

            ProtocolStateProofV1
            ProofStatementV1
            PrevEvalsV1
            ProofV1
            ProofMessagesV1
            ProofOpeningsV1

            StagedLedgerDiffV1
            StagedLedgerDiffTupleV1
            StagedLedgerPreDiffV1

            DeltaTransitionChainProof

            Option<ProtocolVersionV1>
            ()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn ensure_non_exhaustive_deserialization() {
        let mut rng = StdRng::from_seed([0; 32]);
        let mut bytes = vec![0; rng.gen_range((1024 * 1024)..(50 * 1024 * 1024))];

        bytes[0] = 1;
        let mut de = Deserializer::from_reader(bytes.as_slice());
        let _et: ProtocolVersionV1 = Deserialize::deserialize(&mut de).unwrap();
    }

    #[macro_export]
    macro_rules! fuzz_test {
        ($($ty: ty) *) => {
            $(
                let mut rng = StdRng::from_seed([0; 32]);
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
