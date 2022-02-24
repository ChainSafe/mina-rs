// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use super::*;
    use ark_ec::AffineCurve;
    use ark_ff::BigInteger256;
    use mina_crypto::{hash::*, mina_signer::*};
    use mina_curves::pasta::vesta;
    use mina_rs_base::numbers::*;
    use num::BigUint;
    use oracle::poseidon::*;

    #[test]
    pub fn test_amount_to_formatted_string() {
        assert_eq!(Amount(0).to_string(), "0.000000000");
        assert_eq!(Amount(3).to_string(), "0.000000003");
        assert_eq!(Amount(1000000003).to_string(), "1.000000003");
        assert_eq!(Amount(1000000030).to_string(), "1.000000030");
        assert_eq!(Amount(1300000000).to_string(), "1.300000000");
        assert_eq!(Amount(1000000000).to_string(), "1.000000000");
    }

    #[test]
    fn test_convert_bigint_to_arkworks_zero() {
        use ark_ff::BigInteger256;
        let i = BigInt256([0; 32]);
        let ark_i: BigInteger256 = i.into();
        assert_eq!(ark_i, BigInteger256::default())
    }

    // Test cases are generated from OCaml code.
    // Add below lines to src/nonconsensus/mina_numbers/length.ml
    // then run `dune test`
    //
    // let%test_unit "test_length_to_random_oracle_input" =
    //   let fields = 10 |> Unsigned.UInt32.of_int |> of_uint32 |> to_input |> Random_oracle.pack_input in
    //   for i = 0 to Array.length fields - 1 do
    //     Printf.printf "\"%s\",\n" (fields.(i) |> Snark_params.Tick.Field.to_string )
    //   done ;
    //   Printf.printf "\"%s\",\n" (fields |> Random_oracle.hash |> to_string)
    //
    #[test]
    fn test_numbers_to_roinput() {
        test_numbers_to_roinput!(
            0,
            "10810255668636942098026103766265049994195917059170783454356350086236922262043"
        );
        test_numbers_to_roinput!(
            10,
            "17356572411680406737150672960050062949049412030079764528419230045894285941469"
        );
        test_numbers_to_roinput!(
            6666,
            "5166131045352968095909356988540174848079609264277513654820151010925011625414"
        );
    }

    #[macro_export]
    macro_rules! test_numbers_to_roinput {
        ($num:expr, $expected_hash_str:expr) => {
            let fields = {
                let mut roi = ROInput::new();
                Length($num).add_self_to(&mut roi);
                roi.to_fields()
            };
            let mut hasher = ArithmeticSponge::<
                <vesta::Affine as AffineCurve>::ScalarField,
                PlonkSpongeConstantsBasic,
            >::new(oracle::pasta::fp::params());
            hasher.absorb(&fields);
            let hash = hasher.squeeze();
            let big256: BigInteger256 = hash.into();
            let big: BigUint = big256.into();
            assert_eq!($expected_hash_str, big.to_str_radix(10))
        };
    }
}
