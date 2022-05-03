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

use mina_serialization_types::delta_transition_chain_proof::DeltaTransitionChainProof as DeltaTransitionChainProofV1;

impl_from_with_proxy!(
    crate::types::DeltaTransitionChainProof,
    DeltaTransitionChainProofV1,
    DeltaTransitionChainProofJson
);
