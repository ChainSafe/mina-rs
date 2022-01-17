// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_crypto::prelude::*;
    use mina_rs_base::types::*;

    #[test]
    fn test_field_element_vec_roundtrip() {
        let hex_str = "16eba2ebda9feac442e29ef9293f5c4576933d531a6e3c07518e352241055f3d";
        let v = FieldElementVec::try_from_hex(hex_str).unwrap();
        assert_eq!(v.to_hex_string(), hex_str);

        let v = FieldElementVec::try_from_hex(format!("0x{}", hex_str)).unwrap();
        assert_eq!(v.to_hex_string(), hex_str);

        let v = FieldElementVec::try_from_hex(format!("\\x{}", hex_str)).unwrap();
        assert_eq!(v.to_hex_string(), hex_str);

        FieldElementVec::try_from_hex(format!("8x{}", hex_str)).expect_err("error expected");
    }

    #[test]
    fn test_field_element_vec_2_roundtrip() {
        let hex_strs = [
            "717115e59713c84f88babe2ec0292518060d2cc82b54e9a9c9a2d2a87ce91e15",
            "6994e270f284a557c418afebfaaca2794c8af6a476cb1b9478c205e8a901170f",
        ];
        let v = FieldElementVec::try_from_hex(hex_strs.join("")).unwrap();
        assert_eq!(v.0.len(), hex_strs.len());
        for i in 0..hex_strs.len() {
            assert_eq!(hex::encode(v.0[i].0), hex_strs[i]);
        }
    }
}
