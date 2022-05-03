// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::{json::*, v1::*};
use versioned::*;

mod numbers;

impl_from_with_proxy!(
    ExternalTransition,
    ExternalTransitionV1,
    ExternalTransitionJson
);

impl_from_with_proxy!(
    ProtocolStateBody,
    ProtocolStateBodyV1,
    ProtocolStateBodyJson
);

impl_from_with_proxy!(ProtocolState, ProtocolStateV1, ProtocolStateJson);

impl_from_with_proxy!(StagedLedgerDiff, StagedLedgerDiffV1, StagedLedgerDiffJson);

impl From<StagedLedgerPreDiffOne> for StagedLedgerPreDiffOneV1 {
    fn from(t: StagedLedgerPreDiffOne) -> Self {
        StagedLedgerPreDiffOneV1::new(Versioned::new(
            mina_serialization_types::staged_ledger_diff::StagedLedgerPreDiffOne {
                completed_works: t.completed_works.into_iter().map(Into::into).collect(),
                commands: t.commands.into_iter().map(Into::into).collect(),
                coinbase: t.coinbase.into(),
                internal_command_balances: t
                    .internal_command_balances
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            },
        ))
    }
}
impl From<StagedLedgerPreDiffOneV1> for StagedLedgerPreDiffOne {
    fn from(t: StagedLedgerPreDiffOneV1) -> Self {
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

impl From<TransactionSnarkWork> for TransactionSnarkWorkV1 {
    fn from(t: TransactionSnarkWork) -> Self {
        TransactionSnarkWorkV1::new(mina_serialization_types::snark_work::TransactionSnarkWork {
            fee: t.fee.into(),
            proofs: t.proofs.into(),
            prover: t.prover.into(),
        })
    }
}
impl From<TransactionSnarkWorkV1> for TransactionSnarkWork {
    fn from(t: TransactionSnarkWorkV1) -> Self {
        Self {
            fee: t.t.fee.t.t.into(),
            proofs: t.t.proofs.into(),
            prover: t.t.prover.into(),
        }
    }
}

impl From<OneORTwo> for OneORTwoV1 {
    fn from(t: OneORTwo) -> Self {
        use mina_serialization_types::snark_work::OneORTwo as OT;
        match t {
            OneORTwo::One(ts) => {
                let ts_1 = *ts;
                Self::new(OT::One(Box::new(Versioned::new(ts_1.into()))))
            }
            OneORTwo::Two(ts1, ts2) => {
                let ts_1 = *ts1;
                let ts_2 = *ts2;
                Self::new(OT::Two(
                    Box::new(Versioned::new(ts_1.into())),
                    Box::new(Versioned::new(ts_2.into())),
                ))
            }
        }
    }
}
impl From<OneORTwoV1> for OneORTwo {
    fn from(t: OneORTwoV1) -> Self {
        use mina_serialization_types::snark_work::OneORTwo;
        match t.t {
            OneORTwo::One(ts) => Self::One(Box::new(ts.t.into())),
            OneORTwo::Two(ts_1, ts_2) => {
                Self::Two(Box::new(ts_1.t.into()), Box::new(ts_2.t.into()))
            }
        }
    }
}

impl From<TransactionSnark> for TransactionSnarkV1 {
    fn from(t: TransactionSnark) -> Self {
        TransactionSnarkV1::new(mina_serialization_types::snark_work::TransactionSnark {
            statement: t.statement.into(),
            transaction_snark_proof: t.transaction_snark_proof.into(),
        })
    }
}
impl From<TransactionSnarkV1> for TransactionSnark {
    fn from(t: TransactionSnarkV1) -> Self {
        Self {
            statement: t.t.statement.into(),
            transaction_snark_proof: t.t.transaction_snark_proof.into(),
        }
    }
}

impl From<Statement> for StatementV1 {
    fn from(t: Statement) -> Self {
        StatementV1::new(Versioned::new(
            mina_serialization_types::snark_work::Statement {
                source: t.source.into(),
                target: t.target.into(),
                supply_increase: t.supply_increase.into(),
                pending_coinbase_stack_state: t.pending_coinbase_stack_state.into(),
                fee_excess: t.fee_excess.into(),
                next_available_token_before: t.next_available_token_before.into(),
                next_available_token_after: t.next_available_token_after.into(),
                sok_digest: t.sok_digest.into(),
            },
        ))
    }
}
impl From<StatementV1> for Statement {
    fn from(t: StatementV1) -> Self {
        Self {
            source: t.t.t.source.into(),
            target: t.t.t.target.into(),
            supply_increase: t.t.t.supply_increase.t.t.into(),
            pending_coinbase_stack_state: t.t.t.pending_coinbase_stack_state.into(),
            fee_excess: t.t.t.fee_excess.into(),
            next_available_token_before: t.t.t.next_available_token_before.t.t.t.into(),
            next_available_token_after: t.t.t.next_available_token_after.t.t.t.into(),
            sok_digest: t.t.t.sok_digest.t,
        }
    }
}

impl From<PendingCoinbaseStackState> for PendingCoinbaseStackStateV1 {
    fn from(t: PendingCoinbaseStackState) -> Self {
        PendingCoinbaseStackStateV1::new(Versioned::new(
            mina_serialization_types::snark_work::PendingCoinbaseStackState {
                source: t.source.into(),
                target: t.target.into(),
            },
        ))
    }
}
impl From<PendingCoinbaseStackStateV1> for PendingCoinbaseStackState {
    fn from(t: PendingCoinbaseStackStateV1) -> Self {
        Self {
            source: t.t.t.source.into(),
            target: t.t.t.target.into(),
        }
    }
}

impl From<PendingCoinbase> for PendingCoinbaseV1 {
    fn from(t: PendingCoinbase) -> Self {
        PendingCoinbaseV1::new(Versioned::new(
            mina_serialization_types::snark_work::PendingCoinbase {
                data_stack: t.data_stack.into(),
                state_stack: t.state_stack.into(),
            },
        ))
    }
}
impl From<PendingCoinbaseV1> for PendingCoinbase {
    fn from(t: PendingCoinbaseV1) -> Self {
        Self {
            data_stack: t.t.t.data_stack.into(),
            state_stack: t.t.t.state_stack.into(),
        }
    }
}

impl From<StateStack> for StateStackV1 {
    fn from(t: StateStack) -> Self {
        StateStackV1::new(Versioned::new(
            mina_serialization_types::snark_work::StateStack {
                init: t.init.into(),
                curr: t.curr.into(),
            },
        ))
    }
}
impl From<StateStackV1> for StateStack {
    fn from(t: StateStackV1) -> Self {
        Self {
            init: t.t.t.init.into(),
            curr: t.t.t.curr.into(),
        }
    }
}

impl From<FeeExcess> for FeeExcessV1 {
    fn from(t: FeeExcess) -> Self {
        FeeExcessV1::new(Versioned::new(
            mina_serialization_types::snark_work::FeeExcess {
                fee_token_l: t.fee_token_l.into(),
                fee_excess_l: t.fee_excess_l.into(),
                fee_token_r: t.fee_token_r.into(),
                fee_excess_r: t.fee_excess_r.into(),
            },
        ))
    }
}
impl From<FeeExcessV1> for FeeExcess {
    fn from(t: FeeExcessV1) -> Self {
        Self {
            fee_token_l: t.t.t.fee_token_l.t.t.t.into(),
            fee_excess_l: t.t.t.fee_excess_l.into(),
            fee_token_r: t.t.t.fee_token_r.t.t.t.into(),
            fee_excess_r: t.t.t.fee_excess_r.into(),
        }
    }
}

use mina_serialization_types::delta_transition_chain_proof::DeltaTransitionChainProof as DeltaTransitionChainProofV1;

impl_from_with_proxy!(
    crate::types::DeltaTransitionChainProof,
    DeltaTransitionChainProofV1,
    DeltaTransitionChainProofJson
);
