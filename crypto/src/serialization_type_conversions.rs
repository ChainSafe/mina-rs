// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::hash::{NonSnarkStagedLedgerHash, StagedLedgerHash};
use mina_serialization_types::v1::{NonSnarkStagedLedgerHashV1, StagedLedgerHashV1};

impl From<NonSnarkStagedLedgerHash> for NonSnarkStagedLedgerHashV1 {
    fn from(t: NonSnarkStagedLedgerHash) -> Self {
        mina_serialization_types::blockchain_state::NonSnarkStagedLedgerHash {
            ledger_hash: t.ledger_hash.into(),
            aux_hash: t.aux_hash.0.into(),
            pending_coinbase_aux: t.pending_coinbase_aux.0.into(),
        }
        .into()
    }
}
impl From<NonSnarkStagedLedgerHashV1> for NonSnarkStagedLedgerHash {
    fn from(t: NonSnarkStagedLedgerHashV1) -> Self {
        Self {
            ledger_hash: t.t.ledger_hash.into(),
            aux_hash: t.t.aux_hash.t.into(),
            pending_coinbase_aux: t.t.pending_coinbase_aux.t.into(),
        }
    }
}

impl From<StagedLedgerHash> for StagedLedgerHashV1 {
    fn from(t: StagedLedgerHash) -> Self {
        mina_serialization_types::blockchain_state::StagedLedgerHash {
            non_snark: t.non_snark.into(),
            pending_coinbase_hash: t.pending_coinbase_hash.into(),
        }
        .into()
    }
}
impl From<StagedLedgerHashV1> for StagedLedgerHash {
    fn from(t: StagedLedgerHashV1) -> Self {
        Self {
            non_snark: t.t.t.non_snark.into(),
            pending_coinbase_hash: t.t.t.pending_coinbase_hash.t.into(),
        }
    }
}
