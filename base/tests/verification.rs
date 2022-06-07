// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_rs_base::types::ExternalTransition;
    use mina_rs_base::user_commands::SignedCommandPayload;
    use mina_rs_base::verifiable::Verifiable;
    use mina_serialization_types::json::ExternalTransitionJson;
    use proof_systems::mina_signer::{self, NetworkId};
    use test_fixtures::*;

    #[test]
    fn verify_all_binprot_fixtures() {
        let mut ctx = mina_signer::create_legacy::<SignedCommandPayload>(NetworkId::MAINNET);

        assert!(TEST_BLOCKS.iter().all(|(_, v)| {
            let block = ExternalTransition::from(v.external_transitionv1().unwrap());
            block.verify(&mut ctx)
        }))
    }

    #[test]
    fn verify_all_json_fixtures() {
        let mut ctx = mina_signer::create_legacy::<SignedCommandPayload>(NetworkId::MAINNET);

        assert!(JSON_TEST_BLOCKS.iter().all(|(_, v)| {
            let block: ExternalTransition =
                serde_json::from_value::<ExternalTransitionJson>(v.clone())
                    .unwrap()
                    .into();
            block.verify(&mut ctx)
        }))
    }
}
