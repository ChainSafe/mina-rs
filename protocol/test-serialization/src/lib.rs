// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod fuzz;
#[allow(non_snake_case)]
mod test_3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK;

#[cfg(test)]
mod tests {
    use super::{block_path_test, block_path_test_batch};
    use bin_prot::{from_reader, to_writer, Value};
    use mina_crypto::hash::*;
    use mina_crypto::signature::{
        FieldPoint, InnerCurveScalar, PublicKey, PublicKey2, PublicKey3, Signature,
    };
    use mina_rs_base::protocol_state_proof::proof_messages::{
        ProofMessageWithDegreeBound, ProofMessageWithoutDegreeBoundList,
    };
    use mina_rs_base::types::*;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use test_fixtures::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_external_transition() {
        block_path_test_batch! {
            ExternalTransition => ""
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state() {
        block_path_test_batch! {
            ProtocolState => "t/protocol_state"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_previous_state_hash() {
        block_path_test_batch! {
            StateHash => "t/protocol_state/t/t/previous_state_hash"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body() {
        block_path_test_batch! {
            ProtocolStateBody => "t/protocol_state/t/t/body"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body_genesis_state_hash() {
        block_path_test_batch! {
            StateHash => "t/protocol_state/t/t/body/t/t/genesis_state_hash"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body_blockchain_state() {
        block_path_test_batch! {
            SnarkedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/snarked_ledger_hash"
            SnarkedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/genesis_ledger_hash"
            TokenId => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/snarked_next_available_token"
            BlockTime => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/timestamp"
            BlockchainState => "t/protocol_state/t/t/body/t/t/blockchain_state"
        };
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body_blockchain_state_staged_ledger_hash() {
        block_path_test_batch! {
            LedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark/t/ledger_hash"
            AuxHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark/t/aux_hash"
            AuxHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark/t/pending_coinbase_aux"
            NonSnarkStagedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/non_snark"
            CoinBaseHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash/t/t/pending_coinbase_hash"
            StagedLedgerHash => "t/protocol_state/t/t/body/t/t/blockchain_state/t/t/staged_ledger_hash"
        };
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body_consensus_state() {
        block_path_test_batch! {
            Length => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/blockchain_length"
            Length => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/epoch_count"
            Length => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/min_window_density"
            Vec<Length> => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/sub_window_densities"
            VrfOutputTruncated => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/last_vrf_output"
            Amount => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/total_currency"
            GlobalSlot => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/curr_global_slot"
            GlobalSlotNumber => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/global_slot_since_genesis"
            EpochData => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data"
            EpochData => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/next_epoch_data"
            bool => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/has_ancestor_in_same_checkpoint_window"
            PublicKey => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/block_stake_winner"
            ConsensusState => "t/protocol_state/t/t/body/t/t/consensus_state"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body_consensus_state_staking_epoch_data() {
        block_path_test_batch! {
            EpochLedger => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data/t/t/ledger"
            EpochSeed => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data/t/t/seed"
            EpochData => "t/protocol_state/t/t/body/t/t/consensus_state/t/t/staking_epoch_data"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_protocol_state_body_constants() {
        block_path_test_batch! {
            Length => "t/protocol_state/t/t/body/t/t/constants/t/t/k"
            Length => "t/protocol_state/t/t/body/t/t/constants/t/t/slots_per_epoch"
            Length => "t/protocol_state/t/t/body/t/t/constants/t/t/slots_per_sub_window"
            Delta => "t/protocol_state/t/t/body/t/t/constants/t/t/delta"
            BlockTime => "t/protocol_state/t/t/body/t/t/constants/t/t/genesis_state_timestamp"
            ProtocolConstants => "t/protocol_state/t/t/body/t/t/constants"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof() {
        block_path_test_batch! {
            ProtocolStateProof => "t/protocol_state_proof"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_statement() {
        block_path_test_batch! {
            ProofStatement => "t/protocol_state_proof/t/t/t/t/statement"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_statement_proof_state() {
        block_path_test_batch! {
            ProofState => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_statement_proof_state_deferred_values() {
        block_path_test_batch! {
            () => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/bulletproof_challenges/t/t/18"
        }
        block_path_test_batch! {
            BulletproofPreChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/plonk/t/alpha"
            ScalarChallengeVector2 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/plonk/t/beta"
            ScalarChallengeVector2 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/plonk/t/gamma"
            BulletproofPreChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/plonk/t/zeta"
            Plonk => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/plonk"
            ShiftedValue => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/combined_inner_product"
            ShiftedValue => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/b"
            BulletproofPreChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/xi"
            BulletproofChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/bulletproof_challenges/t/t/0"
            BulletproofChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/bulletproof_challenges/t/t/17"
            BulletproofChallengeTuple18 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/bulletproof_challenges"
            Char => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values/t/which_branch"
            ProofStateDeferredValues => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/deferred_values"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_statement_proof_state_sponge_digest_before_evaluations() {
        block_path_test_batch! {
           () => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/sponge_digest_before_evaluations/t/t/4"
        }
        block_path_test_batch! {
            Hex64 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/sponge_digest_before_evaluations/t/t/0"
            Hex64 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/sponge_digest_before_evaluations/t/t/3"
            SpongeDigestBeforeEvaluations => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/sponge_digest_before_evaluations"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_statement_proof_state_me_only() {
        block_path_test_batch! {
            () => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only/t/old_bulletproof_challenges/t/2"
        }
        block_path_test_batch! {
            FiniteECPoint => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only/t/sg"
            BulletproofChallengeTuple17 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only/t/old_bulletproof_challenges/t/0"
            BulletproofChallengeTuple17 => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only/t/old_bulletproof_challenges/t/1"
            ProofStateBulletproofChallenges => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only/t/old_bulletproof_challenges"
            ProofStatePairingBased => "t/protocol_state_proof/t/t/t/t/statement/t/t/proof_state/t/me_only"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_statement_pass_through() {
        block_path_test_batch! {
            () => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/old_bulletproof_challenges/t/0/t/t/18"
        }
        block_path_test_batch! {
            () => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/app_state"
            FiniteECPointVec => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/sg"
            BulletproofPreChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/old_bulletproof_challenges/t/0/t/t/0/t/prechallenge"
            BulletproofChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/old_bulletproof_challenges/t/0/t/t/0"
            BulletproofChallenge => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/old_bulletproof_challenges/t/0/t/t/17"
            BulletproofChallengeTuple18 => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/old_bulletproof_challenges/t/0"
            BulletproofChallenges => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through/t/old_bulletproof_challenges"
            PairingBased => "t/protocol_state_proof/t/t/t/t/statement/t/t/pass_through"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_prev_evals() {
        block_path_test_batch! {
            PrevEvals => "t/protocol_state_proof/t/t/t/t/prev_evals"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_prev_x_hat() {
        block_path_test_batch! {
            PrevXHat => "t/protocol_state_proof/t/t/t/t/prev_x_hat"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_proof() {
        block_path_test_batch! {
            Proof => "t/protocol_state_proof/t/t/t/t/proof"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_proof_messages() {
        block_path_test_batch! {
            ProofMessageWithoutDegreeBoundList => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/l_comm"
            ProofMessageWithoutDegreeBoundList => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/r_comm"
            ProofMessageWithoutDegreeBoundList => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/o_comm"
            ProofMessageWithoutDegreeBoundList => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/z_comm"
            ECPointVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/t_comm/t/unshifted"
            ECPoint => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/t_comm/t/shifted"
            ProofMessageWithDegreeBound => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages/t/t_comm"
            ProofMessages => "t/protocol_state_proof/t/t/t/t/proof/t/t/messages"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_proof_openings() {
        block_path_test_batch! {
            ProofOpenings => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_proof_openings_proof() {
        block_path_test_batch! {
            FiniteECPoint => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/lr/t/0/0"
            FiniteECPoint => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/lr/t/0/1"
            FiniteECPointPairVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/lr"
            BigInt256 => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/z_1"
            BigInt256 => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/z_2"
            FiniteECPoint => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/delta"
            FiniteECPoint => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof/t/sg"
            OpeningProof => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/proof"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_state_proof_proof_openings_evals() {
        type ProofEvaluationsTuple = (ProofEvaluations, ProofEvaluations);
        block_path_test_batch! {
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/l"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/r"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/o"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/z"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/t"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/f"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/sigma1"
            FieldElementVec => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0/t/sigma2"
            ProofEvaluations => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/0"
            ProofEvaluations => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals/1"
            ProofEvaluationsTuple => "t/protocol_state_proof/t/t/t/t/proof/t/t/openings/t/evals"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff() {
        block_path_test_batch! {
            StagedLedgerDiff => "t/staged_ledger_diff"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff() {
        block_path_test_batch! {
            StagedLedgerDiffTuple => "t/staged_ledger_diff/t/diff"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_one() {
        block_path_test_batch! {
            Option<StagedLedgerPreDiffOne> => "t/staged_ledger_diff/t/diff/t/1"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_two() {
        block_path_test_batch! {
            StagedLedgerPreDiffTwo => "t/staged_ledger_diff/t/diff/t/0"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_completed_works() {
        block_path_test_batch! {
            Vec<TransactionSnarkWork> => "t/staged_ledger_diff/t/diff/t/0/t/t/completed_works"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands() {
        block_path_test_batch! {
            UserCommandWithStatus => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0"
            Vec<UserCommandWithStatus> => "t/staged_ledger_diff/t/diff/t/0/t/t/commands"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands_data() {
        block_path_test_batch! {
            SignedCommand => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/[sum]"
            UserCommand => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands_data_payload_common() {
        block_path_test_batch! {
            Amount => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/fee"
            SignedCommandFeeToken => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/fee_token"
            PublicKey2 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/fee_payer_pk"
            ExtendedU32 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/nonce"
            i32 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/valid_until/t/t"
            ExtendedU32 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/valid_until"
            SignedCommandMemo => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common/t/t/t/memo"
            SignedCommandPayloadCommon => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/common"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands_data_payload_body() {
        block_path_test_batch! {
           PublicKey2 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body/t/t/0/t/t/source_pk"
           PublicKey2 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body/t/t/0/t/t/receiver_pk"
           u64 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body/t/t/0/t/t/token_id/t/t/t"
           ExtendedU64_3 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body/t/t/0/t/t/token_id"
           Amount => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body/t/t/0/t/t/amount"
           PaymentPayload => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body/t/t/0"
           SignedCommandPayloadBody => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/payload/t/t/body"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands_data_signer() {
        block_path_test_batch! {
            PublicKey3 => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/signer"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands_data_signature() {
        block_path_test_batch! {
            FieldPoint => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/signature/t/t/0"
            InnerCurveScalar => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/signature/t/t/1"
        }

        block_path_test_batch! {
            (FieldPoint, InnerCurveScalar) => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/signature/t/t"
            Signature => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/0/t/t/signature"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_commands_status() {
        block_path_test_batch! {
            TransactionStatusAuxiliaryData => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/status/t/0"
            TransactionStatusBalanceData => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/status/t/1"
            TransactionStatusApplied => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/status/t/[sum]"
            TransactionStatus => "t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/status"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_coinbase() {
        block_path_test_batch! {
            Option<CoinBaseFeeTransfer> => "t/staged_ledger_diff/t/diff/t/0/t/t/coinbase/t/[sum]"
            CoinBase => "t/staged_ledger_diff/t/diff/t/0/t/t/coinbase"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_staged_ledger_diff_diff_internal_command_balances() {
        block_path_test_batch! {
            CoinBaseBalanceData => "t/staged_ledger_diff/t/diff/t/0/t/t/internal_command_balances/0/t/[sum]"
            FeeTransferBalanceData => "t/staged_ledger_diff/t/diff/t/0/t/t/internal_command_balances/1/t/[sum]"
            InternalCommandBalanceData => "t/staged_ledger_diff/t/diff/t/0/t/t/internal_command_balances/0"
            InternalCommandBalanceData => "t/staged_ledger_diff/t/diff/t/0/t/t/internal_command_balances/1"
            Vec<InternalCommandBalanceData> => "t/staged_ledger_diff/t/diff/t/0/t/t/internal_command_balances"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_delta_transition_chain_proof() {
        block_path_test_batch! {
            StateHash => "t/delta_transition_chain_proof/0"
            Vec<StateHash> => "t/delta_transition_chain_proof/1"
            // FIXME: empty list in current test block
            // StateHash => "t/delta_transition_chain_proof/1/0"
        }
        block_path_test_batch! {
            DeltaTransitionChainProof => "t/delta_transition_chain_proof"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_all_block_subtypes() {
        ////////////////////////////////////////////////////////////////
        // Here is where to add calls to test_in_block for every type
        // that has a strongly typed implementation to test
        ////////////////////////////////////////////////////////////////
        block_path_test_batch! {
            ProtocolVersion => "t/current_protocol_version"
            Option<ProtocolVersion> => "t/proposed_protocol_version_opt"
            StateHash => "t/protocol_state/t/t/previous_state_hash"
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn smoke_test_roundtrip_block1() {
        let block = TEST_BLOCKS.get("block1").expect("Failed to load block1");

        // test we can correctly index a known field
        assert_eq!(
            block.value["t"]["protocol_state"]["t"]["t"]["previous_state_hash"]["t"],
            Value::Tuple(
                vec![
                    30, 76, 197, 215, 115, 43, 42, 245, 198, 30, 253, 134, 49, 117, 82, 71, 182,
                    181, 180, 95, 18, 250, 46, 1, 25, 3, 78, 193, 57, 152, 116, 49,
                ]
                .iter()
                .map(|c| Value::Char(*c))
                .collect()
            )
        );

        // check roundtrip
        test_roundtrip(&block.value, block.bytes.as_slice());
    }

    #[test]
    #[wasm_bindgen_test]
    fn smoke_test_deserialize_block() {
        // check we can deserialize into this type without error
        for (name, block) in TEST_BLOCKS.iter() {
            let et: ExternalTransition = block
                .external_transition()
                .expect("Failed to deserialize block");

            // TODO: Validate state hash
            if name.ends_with(".hex") {}

            // check roundtrip
            test_roundtrip(&et, block.bytes.as_slice());
        }
    }

    fn select_path<'a>(block: &'a bin_prot::Value, path: impl AsRef<str>) -> &'a bin_prot::Value {
        // pull out the bin_prot::Value corresponding to the path
        // will panic if the path is invalid
        let path_ref = path.as_ref();
        if path_ref.len() == 0 {
            return block;
        }
        let mut val = block;
        for p in path_ref.split('/') {
            if p == "[sum]" {
                match val {
                    Value::Sum { ref value, .. } => {
                        val = value;
                    }
                    _ => assert!(false, "Sum expected"),
                }
            } else {
                val = match usize::from_str(p) {
                    Ok(index) => &val[index],
                    _ => &val[p],
                };
            }
        }
        val
    }

    fn test_in_block<'a, T: Serialize + Deserialize<'a>>(block: &bin_prot::Value, paths: &[&str]) {
        for path in paths {
            let val = select_path(block, path);

            // write to binary then deserialize into T
            let mut bytes = vec![];
            bin_prot::to_writer(&mut bytes, val).expect(&format!(
                "Failed writing bin-prot encoded data\npath: {}\ndata: {:#?}",
                path, val
            ));
            let re_val: T = from_reader(bytes.as_slice()).expect(&format!(
                "Could not deserialize type\npath: {}\nbytes({}): {:?}\ndata: {:#?}",
                path,
                bytes.len(),
                bytes,
                val
            ));

            // serialize back to binary and ensure it matches
            let mut re_bytes = vec![];
            to_writer(&mut re_bytes, &re_val).expect(&format!(
                "Failed writing bin-prot encoded data\npath: {}\ndata: {:#?}",
                path, val
            ));

            assert_eq!(bytes, re_bytes, "path: {}\ndata: {:#?}", path, val);
        }
    }

    fn test_roundtrip<T>(val: &T, bytes: &[u8])
    where
        T: Serialize,
    {
        let mut output = vec![];
        bin_prot::to_writer(&mut output, val).expect("Failed writing bin-prot encoded data");
        assert_eq!(bytes, output)
    }

    #[macro_export]
    macro_rules! block_path_test {
        ($typ:ty, $path:expr) => {
            for block in TEST_BLOCKS.values() {
                test_in_block::<$typ>(&block.value, &[$path]);
            }
        };
    }

    #[macro_export]
    macro_rules! block_path_test_batch {
        ($($typ:ty => $path:expr) *)  => {
            $(
                block_path_test!($typ, $path);
            )*
        };
    }
}
