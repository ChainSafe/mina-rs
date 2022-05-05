// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! OCaml polyvars use a special integer output hash function to index variants
//! This module reimplements this hash function

/// Type alias for hash result
pub type VariantHash = u32;

/// Hash the label string (ASCII not UTF-8) into a OCaml style variant hash
pub fn caml_hash_variant(label: &str) -> VariantHash {
    label
        .as_bytes()
        .iter()
        .fold(0_u32, |accu, byte| 223 * accu + (*byte as u32))
}

/* original C code
    CAMLexport value caml_hash_variant(char const * tag)
    {
      value accu;
      /* Same hashing algorithm as in ../typing/btype.ml, function hash_variant */
      for (accu = Val_int(0); *tag != 0; tag++)
        accu = Val_int(223 * Int_val(accu) + *((unsigned char *) tag));
    #ifdef ARCH_SIXTYFOUR
      accu = accu & Val_long(0x7FFFFFFFL);
    #endif
      /* Force sign extension of bit 31 for compatibility between 32 and 64-bit
         platforms */
      return (int32_t) accu;
    }
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_labels() {
        const CASES: &[(&str, u32)] = &[("One", 3953222_u32), ("Two", 4203884_u32)];

        for (label, hash) in CASES {
            assert_eq!(caml_hash_variant(label), *hash)
        }
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_overflow_variant() {
        let _ = caml_hash_variant("Three");
    }
}
