// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]
#![deny(missing_docs)]

pub mod blockchain_state;
pub mod bulletproof_challenges;
pub mod common;
pub mod consensus_state;
pub mod delta_transition_chain_proof;
pub mod epoch_data;
pub mod external_transition;
pub mod field_and_curve_elements;
pub mod global_slot;
pub mod opening_proof;
pub mod proof_evaluations;
pub mod proof_messages;
pub mod protocol_constants;
pub mod protocol_state;
pub mod protocol_state_body;
pub mod protocol_state_proof;
pub mod protocol_version;
pub mod signatures;
pub mod staged_ledger_diff;

/// Version 1 serialization types for the Mina protocol
pub mod v1 {
    pub use super::blockchain_state::{
        BlockchainStateV1, NonSnarkStagedLedgerHashV1, StagedLedgerHashV1,
    };
    pub use super::bulletproof_challenges::{
        BulletproofChallengeTuple17V1, BulletproofChallengeTuple18V1, BulletproofChallengeV1,
        BulletproofChallengesV1, BulletproofPreChallengeV1, ProofStateBulletproofChallengesV1,
        ScalarChallengeVector2V1,
    };
    pub use super::common::{
        AmountV1, BigInt256, BlockTimeV1, ByteVecV1, CharV1, DeltaV1, ExtendedU32, ExtendedU64_2,
        ExtendedU64_3, GlobalSlotNumberV1, Hash2V1, HashV1, Hex64V1, LengthV1, TokenIdV1,
    };
    pub use super::consensus_state::{ConsensusStateV1, VrfOutputTruncatedV1};
    pub use super::delta_transition_chain_proof::DeltaTransitionChainProof;
    pub use super::epoch_data::{EpochDataV1, EpochLedgerV1};
    pub use super::external_transition::ExternalTransitionV1;
    pub use super::field_and_curve_elements::{
        ECPointV1, ECPointVecV1, FieldElement, FieldElementVecV1, FiniteECPoint,
        FiniteECPointPairVecV1, FiniteECPointVecV1,
    };
    pub use super::global_slot::GlobalSlotV1;
    pub use super::opening_proof::OpeningProofV1;
    pub use super::proof_evaluations::ProofEvaluationsV1;
    pub use super::proof_messages::{
        ProofMessageWithDegreeBoundV1, ProofMessageWithoutDegreeBoundListV1, ProofMessagesV1,
    };
    pub use super::protocol_constants::ProtocolConstantsV1;
    pub use super::protocol_state::ProtocolStateV1;
    pub use super::protocol_state_body::ProtocolStateBodyV1;
    pub use super::protocol_state_proof::{
        PairingBasedV1, PlonkV1, PrevEvalsV1, PrevXHatV1, ProofOpeningsV1,
        ProofStateDeferredValuesV1, ProofStatePairingBasedV1, ProofStateV1, ProofStatementV1,
        ProofV1, ProtocolStateProofV1, ShiftedValueV1, SpongeDigestBeforeEvaluationsV1,
    };
    pub use super::protocol_version::ProtocolVersionV1;
    pub use super::signatures::{InnerCurveScalar, PublicKey2V1, PublicKeyV1, SignatureV1};
    pub use super::staged_ledger_diff::{
        CoinBaseBalanceDataV1, CoinBaseFeeTransferV1, CoinBaseV1, FeeTransferBalanceDataV1,
        InternalCommandBalanceDataV1, PaymentPayloadV1, SignedCommandFeeTokenV1,
        SignedCommandMemoV1, SignedCommandPayloadBodyV1, SignedCommandPayloadCommonV1,
        SignedCommandV1, StagedLedgerDiffTupleV1, StagedLedgerDiffV1, StagedLedgerPreDiffOneV1,
        StagedLedgerPreDiffTwoV1, TransactionSnarkWork, TransactionStatusAppliedV1,
        TransactionStatusAuxiliaryDataV1, TransactionStatusBalanceDataV1, TransactionStatusV1,
        UserCommandV1, UserCommandWithStatusV1,
    };
}
