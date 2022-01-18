// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use mina_rs_base::numbers::*;

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
}
