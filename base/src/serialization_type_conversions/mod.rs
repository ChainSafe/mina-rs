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

impl_from_for_versioned_with_proxy!(
    ExternalTransition,
    mina_serialization_types::external_transition::ExternalTransition,
    ExternalTransitionV1
);

impl_from_with_proxy!(
    ExternalTransition,
    ExternalTransitionV1,
    ExternalTransitionJson
);

impl From<BlockchainState> for BlockchainStateV1 {
    fn from(t: BlockchainState) -> Self {
        mina_serialization_types::blockchain_state::BlockchainState {
            staged_ledger_hash: t.staged_ledger_hash.into(),
            snarked_ledger_hash: t.snarked_ledger_hash.into_inner().into(),
            genesis_ledger_hash: t.genesis_ledger_hash.into_inner().into(),
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
            hash: t.hash.into_inner().into(),
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

impl From<ConsensusState> for ConsensusStateV1 {
    fn from(t: ConsensusState) -> Self {
        mina_serialization_types::consensus_state::ConsensusState {
            blockchain_length: t.blockchain_length.into(),
            epoch_count: t.epoch_count.into(),
            min_window_density: t.min_window_density.into(),
            sub_window_densities: t.sub_window_densities.into_iter().map(Into::into).collect(),
            last_vrf_output: t.last_vrf_output.0.into(),
            total_currency: t.total_currency.into(),
            curr_global_slot: t.curr_global_slot.into(),
            global_slot_since_genesis: t.global_slot_since_genesis.into(),
            staking_epoch_data: t.staking_epoch_data.into(),
            next_epoch_data: t.next_epoch_data.into(),
            has_ancestor_in_same_checkpoint_window: t.has_ancestor_in_same_checkpoint_window,
            block_stake_winner: t.block_stake_winner.into(),
            block_creator: t.block_creator.into(),
            coinbase_receiver: t.coinbase_receiver.into(),
            supercharge_coinbase: t.supercharge_coinbase,
        }
        .into()
    }
}
impl From<ConsensusStateV1> for ConsensusState {
    fn from(t: ConsensusStateV1) -> Self {
        Self {
            blockchain_length: t.t.t.blockchain_length.t.t.into(),
            epoch_count: t.t.t.epoch_count.t.t.into(),
            min_window_density: t.t.t.min_window_density.t.t.into(),
            sub_window_densities: t
                .t
                .t
                .sub_window_densities
                .into_iter()
                .map(|v| v.t.t.into())
                .collect(),
            last_vrf_output: t.t.t.last_vrf_output.t.into(),
            total_currency: t.t.t.total_currency.t.t.into(),
            curr_global_slot: t.t.t.curr_global_slot.into(),
            global_slot_since_genesis: t.t.t.global_slot_since_genesis.t.t.into(),
            staking_epoch_data: t.t.t.staking_epoch_data.into(),
            next_epoch_data: t.t.t.next_epoch_data.into(),
            has_ancestor_in_same_checkpoint_window: t.t.t.has_ancestor_in_same_checkpoint_window,
            block_stake_winner: t.t.t.block_stake_winner.into(),
            block_creator: t.t.t.block_creator.into(),
            coinbase_receiver: t.t.t.coinbase_receiver.into(),
            supercharge_coinbase: t.t.t.supercharge_coinbase,
        }
    }
}

impl From<ProtocolConstants> for ProtocolConstantsV1 {
    fn from(t: ProtocolConstants) -> Self {
        mina_serialization_types::protocol_constants::ProtocolConstants {
            k: t.k.into(),
            slots_per_epoch: t.slots_per_epoch.into(),
            slots_per_sub_window: t.slots_per_sub_window.into(),
            delta: t.delta.into(),
            genesis_state_timestamp: t.genesis_state_timestamp.0.into(),
        }
        .into()
    }
}
impl From<ProtocolConstantsV1> for ProtocolConstants {
    fn from(t: ProtocolConstantsV1) -> Self {
        Self {
            k: t.t.t.k.t.t.into(),
            slots_per_epoch: t.t.t.slots_per_epoch.t.t.into(),
            slots_per_sub_window: t.t.t.slots_per_sub_window.t.t.into(),
            delta: t.t.t.delta.t.t.into(),
            genesis_state_timestamp: t.t.t.genesis_state_timestamp.into(),
        }
    }
}

impl From<ProtocolStateBody> for ProtocolStateBodyV1 {
    fn from(t: ProtocolStateBody) -> Self {
        mina_serialization_types::protocol_state_body::ProtocolStateBody {
            genesis_state_hash: t.genesis_state_hash.into(),
            blockchain_state: t.blockchain_state.into(),
            consensus_state: t.consensus_state.into(),
            constants: t.constants.into(),
        }
        .into()
    }
}
impl From<ProtocolStateBodyV1> for ProtocolStateBody {
    fn from(t: ProtocolStateBodyV1) -> Self {
        Self {
            genesis_state_hash: t.t.t.genesis_state_hash.into(),
            blockchain_state: t.t.t.blockchain_state.into(),
            consensus_state: t.t.t.consensus_state.into(),
            constants: t.t.t.constants.into(),
        }
    }
}
impl_from_with_proxy!(
    ProtocolStateBody,
    ProtocolStateBodyV1,
    ProtocolStateBodyJson
);

impl From<ProtocolState> for ProtocolStateV1 {
    fn from(t: ProtocolState) -> Self {
        mina_serialization_types::protocol_state::ProtocolState {
            previous_state_hash: t.previous_state_hash.into(),
            body: t.body.into(),
        }
        .into()
    }
}
impl From<ProtocolStateV1> for ProtocolState {
    fn from(t: ProtocolStateV1) -> Self {
        Self {
            previous_state_hash: t.t.t.previous_state_hash.into(),
            body: t.t.t.body.into(),
        }
    }
}
impl_from_with_proxy!(ProtocolState, ProtocolStateV1, ProtocolStateJson);

impl From<PaymentPayload> for PaymentPayloadV1 {
    fn from(t: PaymentPayload) -> Self {
        mina_serialization_types::staged_ledger_diff::PaymentPayload {
            source_pk: t.source_pk.into(),
            receiver_pk: t.receiver_pk.into(),
            token_id: t.token_id.into(),
            amount: t.amount.into(),
        }
        .into()
    }
}
impl From<PaymentPayloadV1> for PaymentPayload {
    fn from(t: PaymentPayloadV1) -> Self {
        Self {
            source_pk: t.t.t.source_pk.into(),
            receiver_pk: t.t.t.receiver_pk.into(),
            token_id: t.t.t.token_id.t.t.t.into(),
            amount: t.t.t.amount.t.t.into(),
        }
    }
}

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

impl From<SignedCommandPayload> for SignedCommandPayloadV1 {
    fn from(t: SignedCommandPayload) -> Self {
        mina_serialization_types::staged_ledger_diff::SignedCommandPayload {
            common: t.common.into(),
            body: t.body.into(),
        }
        .into()
    }
}
impl From<SignedCommandPayloadV1> for SignedCommandPayload {
    fn from(t: SignedCommandPayloadV1) -> Self {
        Self {
            common: t.t.t.common.into(),
            body: t.t.t.body.into(),
        }
    }
}

impl From<SignedCommand> for SignedCommandV1 {
    fn from(t: SignedCommand) -> Self {
        mina_serialization_types::staged_ledger_diff::SignedCommand {
            payload: t.payload.into(),
            signer: t.signer.into(),
            signature: t.signature.into(),
        }
        .into()
    }
}
impl From<SignedCommandV1> for SignedCommand {
    fn from(t: SignedCommandV1) -> Self {
        Self {
            payload: t.t.t.payload.into(),
            signer: t.t.t.signer.into(),
            signature: t.t.t.signature.into(),
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

impl From<TransactionStatusBalanceData> for TransactionStatusBalanceDataV1 {
    fn from(t: TransactionStatusBalanceData) -> Self {
        mina_serialization_types::staged_ledger_diff::TransactionStatusBalanceData {
            fee_payer_balance: t.fee_payer_balance.map(|v| v.0.into()),
            source_balance: t.source_balance.map(|v| v.0.into()),
            receiver_balance: t.receiver_balance.map(|v| v.0.into()),
        }
        .into()
    }
}
impl From<TransactionStatusBalanceDataV1> for TransactionStatusBalanceData {
    fn from(t: TransactionStatusBalanceDataV1) -> Self {
        Self {
            fee_payer_balance: t.t.fee_payer_balance.map(|v| v.t.t.t.into()),
            source_balance: t.t.source_balance.map(|v| v.t.t.t.into()),
            receiver_balance: t.t.receiver_balance.map(|v| v.t.t.t.into()),
        }
    }
}

impl From<TransactionStatusAuxiliaryData> for TransactionStatusAuxiliaryDataV1 {
    fn from(t: TransactionStatusAuxiliaryData) -> Self {
        mina_serialization_types::staged_ledger_diff::TransactionStatusAuxiliaryData {
            fee_payer_account_creation_fee_paid: t
                .fee_payer_account_creation_fee_paid
                .map(Into::into),
            receiver_account_creation_fee_paid: t
                .receiver_account_creation_fee_paid
                .map(Into::into),
            created_token: t.created_token.map(Into::into),
        }
        .into()
    }
}
impl From<TransactionStatusAuxiliaryDataV1> for TransactionStatusAuxiliaryData {
    fn from(t: TransactionStatusAuxiliaryDataV1) -> Self {
        Self {
            fee_payer_account_creation_fee_paid: t
                .t
                .fee_payer_account_creation_fee_paid
                .map(|v| v.t.t.into()),
            receiver_account_creation_fee_paid: t
                .t
                .receiver_account_creation_fee_paid
                .map(|v| v.t.t.into()),
            created_token: t.t.created_token.map(|v| v.t.t.t.into()),
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

impl From<UserCommandWithStatus> for UserCommandWithStatusV1 {
    fn from(t: UserCommandWithStatus) -> Self {
        mina_serialization_types::staged_ledger_diff::UserCommandWithStatus {
            data: t.data.into(),
            status: t.status.into(),
        }
        .into()
    }
}
impl From<UserCommandWithStatusV1> for UserCommandWithStatus {
    fn from(t: UserCommandWithStatusV1) -> Self {
        Self {
            data: t.t.data.into(),
            status: t.t.status.into(),
        }
    }
}

impl From<CoinBaseFeeTransfer> for CoinBaseFeeTransferV1 {
    fn from(t: CoinBaseFeeTransfer) -> Self {
        mina_serialization_types::staged_ledger_diff::CoinBaseFeeTransfer {
            receiver_pk: t.receiver_pk.into(),
            fee: t.fee.into(),
        }
        .into()
    }
}
impl From<CoinBaseFeeTransferV1> for CoinBaseFeeTransfer {
    fn from(t: CoinBaseFeeTransferV1) -> Self {
        Self {
            receiver_pk: t.t.t.receiver_pk.into(),
            fee: t.t.t.fee.t.t.into(),
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

impl From<CoinBaseBalanceData> for CoinBaseBalanceDataV1 {
    fn from(t: CoinBaseBalanceData) -> Self {
        mina_serialization_types::staged_ledger_diff::CoinBaseBalanceData {
            coinbase_receiver_balance: t.coinbase_receiver_balance.0.into(),
            fee_transfer_receiver_balance: t.fee_transfer_receiver_balance.map(|v| v.0.into()),
        }
        .into()
    }
}
impl From<CoinBaseBalanceDataV1> for CoinBaseBalanceData {
    fn from(t: CoinBaseBalanceDataV1) -> Self {
        Self {
            coinbase_receiver_balance: t.t.coinbase_receiver_balance.t.t.t.into(),
            fee_transfer_receiver_balance: t
                .t
                .fee_transfer_receiver_balance
                .map(|v| v.t.t.t.into()),
        }
    }
}

impl From<FeeTransferBalanceData> for FeeTransferBalanceDataV1 {
    fn from(t: FeeTransferBalanceData) -> Self {
        mina_serialization_types::staged_ledger_diff::FeeTransferBalanceData {
            receiver1_balance: t.receiver1_balance.0.into(),
            receiver2_balance: t.receiver2_balance.map(|v| v.0.into()),
        }
        .into()
    }
}
impl From<FeeTransferBalanceDataV1> for FeeTransferBalanceData {
    fn from(t: FeeTransferBalanceDataV1) -> Self {
        Self {
            receiver1_balance: t.t.receiver1_balance.t.t.t.into(),
            receiver2_balance: t.t.receiver2_balance.map(|v| v.t.t.t.into()),
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

impl From<StagedLedgerPreDiffTwo> for StagedLedgerPreDiffTwoV1 {
    fn from(t: StagedLedgerPreDiffTwo) -> Self {
        mina_serialization_types::staged_ledger_diff::StagedLedgerPreDiffTwo {
            completed_works: t.completed_works.into_iter().map(Into::into).collect(),
            commands: t.commands.into_iter().map(Into::into).collect(),
            coinbase: t.coinbase.into(),
            internal_command_balances: t
                .internal_command_balances
                .into_iter()
                .map(Into::into)
                .collect(),
        }
        .into()
    }
}
impl From<StagedLedgerPreDiffTwoV1> for StagedLedgerPreDiffTwo {
    fn from(t: StagedLedgerPreDiffTwoV1) -> Self {
        Self {
            completed_works: t.t.t.completed_works.into_iter().map(Into::into).collect(),
            commands: t.t.t.commands.into_iter().map(Into::into).collect(),
            coinbase: t.t.t.coinbase.into(),
            internal_command_balances: t
                .t
                .t
                .internal_command_balances
                .into_iter()
                .map(Into::into)
                .collect(),
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

impl From<StagedLedgerDiff> for StagedLedgerDiffV1 {
    fn from(t: StagedLedgerDiff) -> Self {
        mina_serialization_types::staged_ledger_diff::StagedLedgerDiff {
            diff: t.diff.into(),
        }
        .into()
    }
}
impl From<StagedLedgerDiffV1> for StagedLedgerDiff {
    fn from(t: StagedLedgerDiffV1) -> Self {
        StagedLedgerDiff {
            diff: t.t.diff.into(),
        }
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

impl From<ProtocolVersion> for ProtocolVersionV1 {
    fn from(t: ProtocolVersion) -> Self {
        mina_serialization_types::protocol_version::ProtocolVersion::from(t).into()
    }
}
impl From<ProtocolVersionV1> for ProtocolVersion {
    fn from(t: ProtocolVersionV1) -> Self {
        t.t.into()
    }
}
