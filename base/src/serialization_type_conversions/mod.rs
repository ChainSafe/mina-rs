use crate::types::*;
use mina_serialization_types::v1::*;
use versioned::Versioned;

impl From<ExternalTransition> for ExternalTransitionV1 {
    fn from(t: ExternalTransition) -> Self {
        ExternalTransitionV1(Versioned::new(
            mina_serialization_types::external_transition::ExternalTransition {
                protocol_state: t.protocol_state.into(),
                protocol_state_proof: t.protocol_state_proof.into(),
                staged_ledger_diff: t.staged_ledger_diff.into(),
                delta_transition_chain_proof: t.delta_transition_chain_proof.into(),
                current_protocol_version: t.current_protocol_version.into(),
                proposed_protocol_version_opt: t.proposed_protocol_version_opt.map(Into::into),
                validation_callback: (),
            },
        ))
    }
}
impl From<ExternalTransitionV1> for ExternalTransition {
    fn from(t: ExternalTransitionV1) -> Self {
        Self {
            protocol_state: t.0.t.protocol_state.into(),
            protocol_state_proof: t.0.t.protocol_state_proof.into(),
            staged_ledger_diff: t.0.t.staged_ledger_diff.into(),
            delta_transition_chain_proof: t.0.t.delta_transition_chain_proof.into(),
            current_protocol_version: t.0.t.current_protocol_version.into(),
            proposed_protocol_version_opt: t.0.t.proposed_protocol_version_opt.map(Into::into),
        }
    }
}

impl From<SignedCommand> for SignedCommandV1 {
    fn from(t: SignedCommand) -> Self {
        SignedCommandV1::new(
            mina_serialization_types::staged_ledger_diff::SignedCommand {
                payload: t.payload.into(),
                signer: t.signer.into(),
                signature: t.signature.into(),
            },
        )
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
            UserCommand::SignedCommand(sc) => {
                Self::new(Versioned::new(UC::SignedCommand(sc.into())))
            }
            _ => unimplemented!(),
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

impl From<UserCommandWithStatus> for UserCommandWithStatusV1 {
    fn from(t: UserCommandWithStatus) -> Self {
        UserCommandWithStatusV1::new(
            mina_serialization_types::staged_ledger_diff::UserCommandWithStatus {
                data: t.data.into(),
                status: t.status.into(),
            },
        )
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

impl From<StagedLedgerPreDiffTwo> for StagedLedgerPreDiffTwoV1 {
    fn from(t: StagedLedgerPreDiffTwo) -> Self {
        StagedLedgerPreDiffTwoV1::new(Versioned::new(
            mina_serialization_types::staged_ledger_diff::StagedLedgerPreDiffTwo {
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
        StagedLedgerDiffTupleV1::new((t.0 .0.into(), t.0 .1.into()))
    }
}
impl From<StagedLedgerDiffTupleV1> for StagedLedgerDiffTuple {
    fn from(t: StagedLedgerDiffTupleV1) -> Self {
        StagedLedgerDiffTuple((t.t.0.into(), t.t.1.into()))
    }
}

impl From<StagedLedgerDiff> for StagedLedgerDiffV1 {
    fn from(t: StagedLedgerDiff) -> Self {
        StagedLedgerDiffV1::new(
            mina_serialization_types::staged_ledger_diff::StagedLedgerDiff {
                diff: t.diff.into(),
            },
        )
    }
}
impl From<StagedLedgerDiffV1> for StagedLedgerDiff {
    fn from(t: StagedLedgerDiffV1) -> Self {
        StagedLedgerDiff {
            diff: t.t.diff.into(),
        }
    }
}

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

impl From<ProtocolVersion> for ProtocolVersionV1 {
    fn from(t: ProtocolVersion) -> Self {
        ProtocolVersionV1::new(
            mina_serialization_types::protocol_version::ProtocolVersion {
                major: t.major,
                minor: t.minor,
                patch: t.patch,
            },
        )
    }
}
impl From<ProtocolVersionV1> for ProtocolVersion {
    fn from(t: ProtocolVersionV1) -> Self {
        ProtocolVersion {
            major: t.t.major,
            minor: t.t.minor,
            patch: t.t.patch,
        }
    }
}
