// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_serialization_types::traits::StrConv;
    use proof_systems::mina_signer::CompressedPubKey;

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
        let pk = CompressedPubKey::from_str(pubkey)?;
        let pubkey_from_key = pk.try_into_string()?;
        assert_eq!(pubkey, &pubkey_from_key);
        Ok(())
    }
}
