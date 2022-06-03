// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_rs_base::types::ExternalTransition;
    use mina_rs_base::user_commands::SignedCommandPayload;
    use mina_rs_base::verifiable::Verifiable;
    use proof_systems::mina_signer::{self, NetworkId};
    use test_fixtures::*;

    #[test]
    fn verify_all_fixtures() {
        let mut ctx = mina_signer::create_legacy::<SignedCommandPayload>(NetworkId::MAINNET);

        assert!(TEST_BLOCKS.iter().all(|(_, v)| {
            let block = ExternalTransition::from(v.external_transitionv1().unwrap());
            block.verify(&mut ctx)
        }))
    }
}
