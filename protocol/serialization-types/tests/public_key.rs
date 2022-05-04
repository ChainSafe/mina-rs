// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_serialization_types::{json::*, signatures::CompressedCurvePoint};

    #[test]
    fn public_key_serde_roundtrip_mainnet() -> anyhow::Result<()> {
        // https://minaexplorer.com/wallet/B62qjCuPisQjLW7YkB22BR9KieSmUZTyApftqxsAuB3U21r3vj1YnaG
        public_key_serde_roundtrip_inner("B62qjCuPisQjLW7YkB22BR9KieSmUZTyApftqxsAuB3U21r3vj1YnaG")
    }

    #[test]
    fn public_key_serde_roundtrip_devnet() -> anyhow::Result<()> {
        // https://devnet.minaexplorer.com/wallet/B62qr1Nx2tDVE2dtq87HCbCRcCEN87gdRLLK5nU2JgammUqbHgB8B2f
        public_key_serde_roundtrip_inner("B62qr1Nx2tDVE2dtq87HCbCRcCEN87gdRLLK5nU2JgammUqbHgB8B2f")
    }

    fn public_key_serde_roundtrip_inner(pubkey: &str) -> anyhow::Result<()> {
        let json_string = serde_json::to_string(pubkey)?;
        let json: PublicKeyJson = serde_json::from_str(&json_string)?;
        let pk: CompressedCurvePoint = json.clone().into();
        let json_from_key: PublicKeyJson = pk.into();
        assert_eq!(json, json_from_key);
        let json_string_from_key = serde_json::to_string(&json_from_key)?;
        assert_eq!(json_string, json_string_from_key);
        let pubkey_from_key: &str = serde_json::from_str(&json_string_from_key)?;
        assert_eq!(pubkey, pubkey_from_key);
        Ok(())
    }
}
