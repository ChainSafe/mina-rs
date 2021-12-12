#[cfg(test)]
mod tests {
    use mina_crypto::ark_ec::{AffineCurve, ProjectiveCurve};
    use mina_crypto::ark_ff::{BigInteger, BigInteger256, FromBytes};
    use mina_crypto::mina_curves::pasta::{pallas, Fq};
    use mina_crypto::{
        base58::Base58Encodable,
        signature::{CompressedCurvePoint, PublicKey},
    };
    use mina_secrets::secret_box::*;

    // Note that file permission validation is not covered in this test case
    const PRIVATE_KEY_JSON: &str = include_str!("data/test-wallet");
    const PUBLIC_KEY: &str = include_str!("data/test-wallet.pub");
    const PASSWORD: &[u8] = include_bytes!("data/test-wallet.pswd");

    #[test]
    fn secret_box_json_round_trip() {
        let sb: SecretBox = PRIVATE_KEY_JSON.try_into().unwrap();
        let value_from_sb: serde_json::Value = sb.try_into().unwrap();
        let value_from_str: serde_json::Value = serde_json::from_str(PRIVATE_KEY_JSON).unwrap();
        assert_eq!(value_from_sb, value_from_str);
    }

    #[test]
    fn secret_box_decrypt_wrong_password() {
        let sb: SecretBox = PRIVATE_KEY_JSON.try_into().unwrap();
        let err = sb.try_get_private_key(b"i_am_wrong_password").unwrap_err();
        match err {
            Error::AeadError(_) => {}
            _ => {
                assert!(false)
            }
        };
    }

    #[test]
    fn secret_box_decrypt() {
        let sb: SecretBox = PRIVATE_KEY_JSON.try_into().unwrap();
        let privkey_bytes = sb.try_get_private_key(PASSWORD).unwrap();
        assert_eq!(privkey_bytes.len(), 32);
        println!(
            "privkey_bytes({}): {:?}",
            privkey_bytes.len(),
            privkey_bytes
        );
        let privkey_scalar = Fq::from(BigInteger256::read(privkey_bytes.as_slice()).unwrap());
        let prime = pallas::Affine::prime_subgroup_generator();
        let public_key_projective = prime.mul(privkey_scalar);
        println!("public_key_projective: {}", public_key_projective);
        let public_key_affine = public_key_projective.into_affine();
        println!("public_key_affine: {}", public_key_affine);
        let x = public_key_affine.x;
        let y = public_key_affine.y;
        let x_big: BigInteger256 = x.into();
        let x_bytes_vec = x_big.to_bytes_le();
        let mut x_bytes = [0; 32];
        x_bytes.copy_from_slice(x_bytes_vec.as_slice());
        let public_key = PublicKey {
            poly: CompressedCurvePoint {
                x: x_bytes,
                is_odd: y.0.get_bit(0),
            },
        };
        assert_eq!(PUBLIC_KEY, public_key.to_base58().into_string());
    }
}
