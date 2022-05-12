// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use proof_systems::*;

    use super::*;
    use ark_ff::BigInteger256;
    use mina_hasher::*;
    use mina_rs_base::numbers::*;
    use num::BigUint;
    use std::str::FromStr;

    #[test]
    pub fn test_amount_to_string() {
        assert_eq!(Amount(0).to_string(), "0.000000000");
        assert_eq!(Amount(3).to_string(), "0.000000003");
        assert_eq!(Amount(1000000003).to_string(), "1.000000003");
        assert_eq!(Amount(1000000030).to_string(), "1.000000030");
        assert_eq!(Amount(1300000000).to_string(), "1.300000000");
        assert_eq!(Amount(1000000000).to_string(), "1.000000000");
    }

    #[test]
    pub fn test_amount_from_string() {
        assert_eq!(Amount::from_str("0.000000000").unwrap(), Amount(0));
        assert_eq!(Amount::from_str("0.000000003").unwrap(), Amount(3));
        assert_eq!(Amount::from_str("1.000000003").unwrap(), Amount(1000000003));
        assert_eq!(Amount::from_str("1.000000030").unwrap(), Amount(1000000030));
        assert_eq!(Amount::from_str("1.00000003").unwrap(), Amount(1000000030));
        assert_eq!(Amount::from_str("1.300000000").unwrap(), Amount(1300000000));
        assert_eq!(Amount::from_str("1.000000000").unwrap(), Amount(1000000000));

        assert_eq!(
            Amount::from_str("0.000000000.0").unwrap_err(),
            ParseAmountError::ErrorInvalidFormat("0.000000000.0".to_string())
        );
        assert_eq!(Amount::from_str("000000000").unwrap(), Amount(0));
    }

    #[test]
    fn test_convert_bigint_to_arkworks_zero() {
        use ark_ff::BigInteger256;
        let i = BigInt256([0; 32]);
        let ark_i: BigInteger256 = i.into();
        assert_eq!(ark_i, BigInteger256::default())
    }

    // Test cases are generated from OCaml code.
    // Add below lines to `src/lib/mina_base/token_id.ml`
    // then run `dune test` under `src/lib/mina_base/`
    //
    /*
    let%test_unit "test_token_id_to_random_oracle_input" =
      let print_hash n =
        let fields =
          n |> Unsigned.UInt64.of_int |> of_uint64 |> to_input
          |> Random_oracle.pack_input
        in
        for i = 0 to Array.length fields - 1 do
          Printf.printf "\"%s\",\n" (fields.(i) |> Snark_params.Tick.Field.to_string)
        done ;
        Printf.printf "\"%s\",\n"
          ( fields |> Random_oracle.hash |> State_hash.of_hash
          |> Snark_params.Tick.Field.to_string )
      in
      print_hash 0 ; print_hash 10 ; print_hash 6666
            */
    #[test]
    fn parity_test_token_id_to_roinput() {
        test_number_to_roinput!(
            TokenId(0),
            "10810255668636942098026103766265049994195917059170783454356350086236922262043"
        );
        test_number_to_roinput!(
            TokenId(10),
            "17356572411680406737150672960050062949049412030079764528419230045894285941469"
        );
        test_number_to_roinput!(
            TokenId(6666),
            "5166131045352968095909356988540174848079609264277513654820151010925011625414"
        );
    }

    /*
    let%test_unit "test_balance_to_random_oracle_input" =
      let print_hash n =
        let fields =
          n |> Currency.Balance.of_int |> Currency.Balance.to_input
          |> Random_oracle.pack_input
        in
        for i = 0 to Array.length fields - 1 do
          Printf.printf "\"%s\",\n" (fields.(i) |> Snark_params.Tick.Field.to_string)
        done ;
        Printf.printf "\"%s\",\n"
          ( fields |> Random_oracle.hash |> State_hash.of_hash
          |> Snark_params.Tick.Field.to_string )
      in
      print_hash 0 ; print_hash 10 ; print_hash 6666

         */
    #[test]
    fn parity_test_amount_to_roinput() {
        test_number_to_roinput!(
            Amount(0),
            "10810255668636942098026103766265049994195917059170783454356350086236922262043"
        );
        test_number_to_roinput!(
            Amount(10),
            "17356572411680406737150672960050062949049412030079764528419230045894285941469"
        );
        test_number_to_roinput!(
            Amount(6666),
            "5166131045352968095909356988540174848079609264277513654820151010925011625414"
        );
    }

    #[macro_export]
    macro_rules! test_number_to_roinput {
        ($num:expr, $expected_hash_str:expr) => {
            let mut hasher = create_legacy(());
            let hash = hasher.hash(&$num);
            let big256: BigInteger256 = hash.into();
            let big: BigUint = big256.into();
            assert_eq!($expected_hash_str, big.to_str_radix(10))
        };
    }
}
