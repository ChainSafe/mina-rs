// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use mina_secrets::secret_box::*;
    use wasm_bindgen_test::*;

    // Note that file permission validation is not covered in this test case
    const PRIVATE_KEY_JSON: &str = include_str!("data/test-wallet");
    const PUBLIC_KEY: &str = include_str!("data/test-wallet.pub");
    const PASSWORD: &[u8] = include_bytes!("data/test-wallet.pswd");

    #[test]
    #[wasm_bindgen_test]
    fn secret_box_json_round_trip() {
        let sb: SecretBox = PRIVATE_KEY_JSON.try_into().unwrap();
        let value_from_sb: serde_json::Value = sb.try_into().unwrap();
        let value_from_str: serde_json::Value = serde_json::from_str(PRIVATE_KEY_JSON).unwrap();
        assert_eq!(value_from_sb, value_from_str);
    }

    #[test]
    #[wasm_bindgen_test]
    fn secret_box_decrypt_wrong_password() {
        let sb: SecretBox = PRIVATE_KEY_JSON.try_into().unwrap();
        let err = sb
            .get_private_key_bytes(b"i_am_wrong_password")
            .unwrap_err();
        match err {
            Error::AeadError(_) => {}
            _ => {
                assert!(false)
            }
        };
    }

    #[test]
    fn secret_box_keypair() -> anyhow::Result<()> {
        let sb: SecretBox = PRIVATE_KEY_JSON.try_into()?;
        let keypair = sb.get_keypair(PASSWORD)?;
        let pk = keypair.public.into_compressed();
        assert_eq!(pk.into_address().as_str(), PUBLIC_KEY);
        Ok(())
    }

    #[wasm_bindgen_test]
    fn secret_box_keypair_wasm() {
        secret_box_keypair().unwrap();
    }
}
