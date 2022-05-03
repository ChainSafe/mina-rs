// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::{json::*, v1::*};
use versioned::*;

mod account;
mod bulletproof_challenges;
mod field_and_curve_elements;
mod numbers;
mod protocol_state_proof;

impl_from_with_proxy!(
    ExternalTransition,
    ExternalTransitionV1,
    ExternalTransitionJson
);

impl From<BlockchainState> for BlockchainStateV1 {
    fn from(t: BlockchainState) -> Self {
        mina_serialization_types::blockchain_state::BlockchainState {
            staged_ledger_hash: t.staged_ledger_hash.into(),
            snarked_ledger_hash: t.snarked_ledger_hash.into(),
            genesis_ledger_hash: t.genesis_ledger_hash.into(),
            snarked_next_available_token: t.snarked_next_available_token.into(),
            timestamp: t.timestamp.0.into(),
        }
        .into()
    }
}
impl From<BlockchainStateV1> for BlockchainState {
    fn from(t: BlockchainStateV1) -> Self {
        Self {
            staged_ledger_hash: t.t.t.staged_ledger_hash.into(),
            snarked_ledger_hash: t.t.t.snarked_ledger_hash.into(),
            genesis_ledger_hash: t.t.t.genesis_ledger_hash.into(),
            snarked_next_available_token: t.t.t.snarked_next_available_token.t.t.t.into(),
            timestamp: t.t.t.timestamp.into(),
        }
    }
}

impl From<GlobalSlot> for GlobalSlotV1 {
    fn from(t: GlobalSlot) -> Self {
        mina_serialization_types::global_slot::GlobalSlot {
            slot_number: t.slot_number.into(),
            slots_per_epoch: t.slots_per_epoch.into(),
        }
        .into()
    }
}
impl From<GlobalSlotV1> for GlobalSlot {
    fn from(t: GlobalSlotV1) -> Self {
        Self {
            slot_number: t.t.t.slot_number.t.t.into(),
            slots_per_epoch: t.t.t.slots_per_epoch.t.t.into(),
        }
    }
}

impl From<EpochLedger> for EpochLedgerV1 {
    fn from(t: EpochLedger) -> Self {
        mina_serialization_types::epoch_data::EpochLedger {
            hash: t.hash.into(),
            total_currency: t.total_currency.into(),
        }
        .into()
    }
}
impl From<EpochLedgerV1> for EpochLedger {
    fn from(t: EpochLedgerV1) -> Self {
        Self {
            hash: t.t.t.hash.into(),
            total_currency: t.t.t.total_currency.t.t.into(),
        }
    }
}

impl From<EpochData> for EpochDataV1 {
    fn from(t: EpochData) -> Self {
        mina_serialization_types::epoch_data::EpochData {
            ledger: t.ledger.into(),
            seed: t.seed.into(),
            start_checkpoint: t.start_checkpoint.into(),
            lock_checkpoint: t.lock_checkpoint.into(),
            epoch_length: t.epoch_length.into(),
        }
        .into()
    }
}
impl From<EpochDataV1> for EpochData {
    fn from(t: EpochDataV1) -> Self {
        Self {
            ledger: t.t.t.ledger.into(),
            seed: t.t.t.seed.into(),
            start_checkpoint: t.t.t.start_checkpoint.into(),
            lock_checkpoint: t.t.t.lock_checkpoint.into(),
            epoch_length: t.t.t.epoch_length.t.t.into(),
        }
    }
}

impl_from_with_proxy!(
    ProtocolStateBody,
    ProtocolStateBodyV1,
    ProtocolStateBodyJson
);

impl_from_with_proxy!(ProtocolState, ProtocolStateV1, ProtocolStateJson);

impl From<SignedCommandPayloadBody> for SignedCommandPayloadBodyV1 {
    fn from(t: SignedCommandPayloadBody) -> Self {
        use mina_serialization_types::staged_ledger_diff::SignedCommandPayloadBody as b;
        match t {
            SignedCommandPayloadBody::PaymentPayload(pp) => b::PaymentPayload(pp.into()).into(),
        }
    }
}
impl From<SignedCommandPayloadBodyV1> for SignedCommandPayloadBody {
    fn from(t: SignedCommandPayloadBodyV1) -> Self {
        use mina_serialization_types::staged_ledger_diff::SignedCommandPayloadBody as b;
        match t.t.t {
            b::PaymentPayload(pp) => Self::PaymentPayload(pp.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<SignedCommandMemo> for SignedCommandMemoV1 {
    fn from(t: SignedCommandMemo) -> Self {
        Self::new(t.0)
    }
}

impl From<SignedCommandPayloadCommon> for SignedCommandPayloadCommonV1 {
    fn from(t: SignedCommandPayloadCommon) -> Self {
        mina_serialization_types::staged_ledger_diff::SignedCommandPayloadCommon {
            fee: t.fee.into(),
            fee_token: t.fee_token.into(),
            fee_payer_pk: t.fee_payer_pk.into(),
            nonce: (t.nonce.0 as i32).into(),
            valid_until: (t.valid_until.0 as i32).into(),
            memo: t.memo.into(),
        }
        .into()
    }
}
impl From<SignedCommandPayloadCommonV1> for SignedCommandPayloadCommon {
    fn from(t: SignedCommandPayloadCommonV1) -> Self {
        Self {
            fee: t.t.t.t.fee.t.t.into(),
            fee_token: t.t.t.t.fee_token.t.t.t.into(),
            fee_payer_pk: t.t.t.t.fee_payer_pk.into(),
            nonce: AccountNonce(t.t.t.t.nonce.t.t as u32), // TODO - remove these casts once ExtendedU32 is properly handled
            valid_until: GlobalSlotNumber(t.t.t.t.valid_until.t.t as u32),
            memo: t.t.t.t.memo.t.into(),
        }
    }
}

impl From<UserCommand> for UserCommandV1 {
    fn from(t: UserCommand) -> Self {
        use mina_serialization_types::staged_ledger_diff::UserCommand as UC;
        match t {
            UserCommand::SignedCommand(sc) => UC::SignedCommand(sc.into()).into(),
        }
    }
}
impl From<UserCommandV1> for UserCommand {
    fn from(t: UserCommandV1) -> Self {
        use mina_serialization_types::staged_ledger_diff::UserCommand as UC;
        match t.t.t {
            UC::SignedCommand(sc) => Self::SignedCommand(sc.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<TransactionStatusApplied> for TransactionStatusAppliedV1 {
    fn from(t: TransactionStatusApplied) -> Self {
        TransactionStatusAppliedV1((t.0 .0.into(), t.0 .1.into()))
    }
}
impl From<TransactionStatusAppliedV1> for TransactionStatusApplied {
    fn from(t: TransactionStatusAppliedV1) -> Self {
        Self((t.0 .0.into(), t.0 .1.into()))
    }
}

impl From<TransactionStatus> for TransactionStatusV1 {
    fn from(t: TransactionStatus) -> Self {
        use mina_serialization_types::staged_ledger_diff::TransactionStatus as TS;
        match t {
            TransactionStatus::Applied(sc) => Self::new(TS::Applied(sc.into())),
        }
    }
}
impl From<TransactionStatusV1> for TransactionStatus {
    fn from(t: TransactionStatusV1) -> Self {
        use mina_serialization_types::staged_ledger_diff::TransactionStatus as TS;
        match t.t {
            TS::Applied(a) => Self::Applied(a.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<CoinBase> for CoinBaseV1 {
    fn from(t: CoinBase) -> Self {
        use mina_serialization_types::staged_ledger_diff::CoinBase as CB;
        match t {
            CoinBase::Zero => Self::new(CB::Zero),
            CoinBase::One(maybe_fee) => Self::new(CB::One(maybe_fee.map(Into::into))),
            CoinBase::Two => Self::new(CB::Two),
        }
    }
}
impl From<CoinBaseV1> for CoinBase {
    fn from(t: CoinBaseV1) -> Self {
        use mina_serialization_types::staged_ledger_diff::CoinBase as CB;
        match t.t {
            CB::Zero => Self::Zero,
            CB::One(maybe_fee) => Self::One(maybe_fee.map(Into::into)),
            CB::Two => Self::Two,
            _ => unimplemented!(),
        }
    }
}

impl From<InternalCommandBalanceData> for InternalCommandBalanceDataV1 {
    fn from(t: InternalCommandBalanceData) -> Self {
        use mina_serialization_types::staged_ledger_diff::InternalCommandBalanceData as BD;
        match t {
            InternalCommandBalanceData::CoinBase(data) => Self::new(BD::CoinBase(data.into())),
            InternalCommandBalanceData::FeeTransfer(data) => {
                Self::new(BD::FeeTransfer(data.into()))
            }
        }
    }
}
impl From<InternalCommandBalanceDataV1> for InternalCommandBalanceData {
    fn from(t: InternalCommandBalanceDataV1) -> Self {
        use mina_serialization_types::staged_ledger_diff::InternalCommandBalanceData as BD;
        match t.t {
            BD::CoinBase(data) => Self::CoinBase(data.into()),
            BD::FeeTransfer(data) => Self::FeeTransfer(data.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<StagedLedgerDiffTuple> for StagedLedgerDiffTupleV1 {
    fn from(t: StagedLedgerDiffTuple) -> Self {
        (t.0 .0.into(), t.0 .1).into()
    }
}
impl From<StagedLedgerDiffTupleV1> for StagedLedgerDiffTuple {
    fn from(t: StagedLedgerDiffTupleV1) -> Self {
        StagedLedgerDiffTuple((t.t.0.into(), t.t.1))
    }
}

impl_from_with_proxy!(StagedLedgerDiff, StagedLedgerDiffV1, StagedLedgerDiffJson);

use mina_serialization_types::delta_transition_chain_proof::DeltaTransitionChainProof as DeltaTransitionChainProofV1;

impl From<crate::types::DeltaTransitionChainProof> for DeltaTransitionChainProofV1 {
    fn from(t: crate::types::DeltaTransitionChainProof) -> Self {
        Self(t.0.into(), t.1.into_iter().map(Into::into).collect())
    }
}
impl From<DeltaTransitionChainProofV1> for crate::types::DeltaTransitionChainProof {
    fn from(t: DeltaTransitionChainProofV1) -> Self {
        Self(t.0.into(), t.1.into_iter().map(Into::into).collect())
    }
}
impl_from_with_proxy!(
    crate::types::DeltaTransitionChainProof,
    DeltaTransitionChainProofV1,
    DeltaTransitionChainProofJson
);
