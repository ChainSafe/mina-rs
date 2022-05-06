// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This crate provides a number of types that capture shape of the data structures used by Mina protocol
//! for communicating between nodes.
//!
//! When used with the serde enabled [bin-prot](https://crates.io/crates/bin-prot) crate
//! this allows for serializing and deserializing Mina protocol wire messages.
//!
//! This crate contains no code outside of autogenerated serde implementations. It is for reading serialized
//! data into strongly typed structures only.
//!

#![deny(warnings)]
#![deny(missing_docs)]

pub mod account;
pub mod blockchain_state;
pub mod bulletproof_challenges;
pub mod common;
pub mod consensus_state;
pub mod delta_transition_chain_proof;
pub mod epoch_data;
pub mod errors;
pub mod external_transition;
pub mod field_and_curve_elements;
pub mod global_slot;
pub mod macros;
pub mod opening_proof;
pub mod proof_evaluations;
pub mod proof_messages;
pub mod protocol_constants;
pub mod protocol_state;
pub mod protocol_state_body;
pub mod protocol_state_proof;
pub mod protocol_version;
pub mod signatures;
pub mod snark_work;
pub mod staged_ledger_diff;
pub mod version_bytes;

mod type_annotations;
pub use type_annotations::*;

/// Version 1 serialization types for the Mina protocol
pub mod v1 {
    pub use super::account::{
        AccountV1, AuthRequiredV1, PermissionsV1, TimingV1, TokenPermissionsV1,
    };
    pub use super::blockchain_state::{
        BlockchainStateV1, NonSnarkStagedLedgerHashV1, StagedLedgerHashV1,
    };
    pub use super::bulletproof_challenges::{
        BulletproofChallengeTuple17V1, BulletproofChallengeTuple18V1, BulletproofChallengeV1,
        BulletproofChallengesV1, BulletproofPreChallengeV1, ProofStateBulletproofChallengesV1,
        ScalarChallengeVector2V1,
    };
    pub use super::common::{
        AccountNonceV1, AmountV1, BigInt256, BlockTimeV1, ByteVecV1, CharV1, DeltaV1, ExtendedU32,
        ExtendedU64, ExtendedU64_2, ExtendedU64_3, GlobalSlotNumberV1, Hash2V1, HashV1, Hex64V1,
        LengthV1, TokenIdV1,
    };
    pub use super::consensus_state::{ConsensusStateV1, VrfOutputTruncatedV1};
    pub use super::delta_transition_chain_proof::DeltaTransitionChainProof;
    pub use super::epoch_data::{EpochDataV1, EpochLedgerV1};
    pub use super::external_transition::ExternalTransitionV1;
    pub use super::field_and_curve_elements::{
        ECPointV1, ECPointVecV1, FieldElement, FieldElementVecV1, FiniteECPoint,
        FiniteECPointPairVecV1, FiniteECPointVecV1, InnerCurveScalar,
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
    pub use super::signatures::{PublicKey2V1, PublicKeyV1, SignatureV1};
    pub use super::snark_work::{
        FeeExcessV1, LedgerProofV1, OneORTwoV1, PendingCoinbaseStackStateV1, PendingCoinbaseV1,
        SgnTypeV1, SignedV1, StateStackV1, StatementV1, TransactionSnarkV1, TransactionSnarkWorkV1,
    };
    pub use super::staged_ledger_diff::{
        CoinBaseBalanceDataV1, CoinBaseFeeTransferV1, CoinBaseV1, FeeTransferBalanceDataV1,
        InternalCommandBalanceDataV1, PaymentPayloadV1, SignedCommandFeeTokenV1,
        SignedCommandMemoV1, SignedCommandPayloadBodyV1, SignedCommandPayloadCommonV1,
        SignedCommandPayloadV1, SignedCommandV1, StagedLedgerDiffTupleV1, StagedLedgerDiffV1,
        StagedLedgerPreDiffOneV1, StagedLedgerPreDiffTwoV1, TransactionStatusAppliedV1,
        TransactionStatusAuxiliaryDataV1, TransactionStatusBalanceDataV1, TransactionStatusV1,
        UserCommandV1, UserCommandWithStatusV1,
    };
}

/// json serialization types for the Mina protocol
pub mod json {
    use super::*;

    pub use blockchain_state::BlockchainStateJson;
    pub use bulletproof_challenges::{
        BulletproofChallengeJson, BulletproofChallengeTuple17Json, BulletproofChallengeTuple18Json,
        BulletproofPreChallengeJson, ProofStateBulletproofChallengesJson,
        ScalarChallengeVector2Json,
    };
    pub use common::{
        AuxHashJson, ChainHashV1Json, CoinBaseHashV1Json, EpochSeedHashV1Json, HashV1Json,
        LedgerHashV1Json, PendingCoinbaseAuxHashJson, StateHashV1Json, VrfOutputHashV1Json,
    };
    pub use consensus_state::{ConsensusStateJson, VrfOutputTruncatedJson};
    pub use delta_transition_chain_proof::DeltaTransitionChainProofJson;
    pub use external_transition::ExternalTransitionJson;
    pub use protocol_constants::ProtocolConstantsJson;
    pub use protocol_state::ProtocolStateJson;
    pub use protocol_state_body::ProtocolStateBodyJson;
    pub use protocol_state_proof::{
        PairingBasedJson, PlonkJson, ProofStateDeferredValuesJson, ProofStateJson,
        ProofStatementJson, ProtocolStateProofBase64Json, ProtocolStateProofJson,
        SpongeDigestBeforeEvaluations,
    };
    pub use protocol_version::ProtocolVersionJson;
    pub use signatures::PublicKeyJson;
    pub use snark_work::{
        FeeExcessJson, PendingCoinbaseJson, PendingCoinbaseStackStateJson, SgnTypeJson, SignedJson,
        StateStackJson, StatementJson, TransactionSnarkJson, TransactionSnarkWorkJson,
    };
    pub use staged_ledger_diff::{CoinBaseJson, StagedLedgerDiffJson};
}
