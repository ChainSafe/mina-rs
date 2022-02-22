use crate::types::*;
use mina_serialization_types::v1::*;
use mina_serialization_types::v1::FiniteECPoint as FiniteECPointV1;
use crate::types::FiniteECPoint;
use versioned::Versioned;

impl From<PairingBased> for PairingBasedV1 {
    fn from(t: PairingBased) -> Self {
        PairingBasedV1::new(
            mina_serialization_types::protocol_state_proof::PairingBased {
                app_state: (),
                sg: t.sg.into(),
                old_bulletproof_challenges: t.old_bulletproof_challenges.into(),
            },
        )
    }
}
impl From<PairingBasedV1> for PairingBased {
    fn from(t: PairingBasedV1) -> Self {
        Self {
            app_state: t.t.app_state.into(),
            sg: t.t.sg.into(),
            old_bulletproof_challenges: t.t.old_bulletproof_challenges.into(),
        }
    }
}

impl From<FiniteECPointVec> for FiniteECPointVecV1 {
    fn from(t: FiniteECPointVec) -> Self {
        Versioned::new(t.0.into_iter().map(Into::into).collect())
    }
}
impl From<FiniteECPointVecV1> for FiniteECPointVec {
    fn from(t: FiniteECPointVecV1) -> Self {
        Self(t.t.into_iter().map(Into::into).collect())
    }
}

impl From<FiniteECPoint> for FiniteECPointV1 {
    fn from(t: FiniteECPoint) -> Self {
        Self (
            t.0.0.into(), t.1.0.into()
        )
    }
}
impl From<FiniteECPointV1> for FiniteECPoint {
    fn from(t: FiniteECPointV1) -> Self {
        Self (
            t.0.into(), t.1.into()
        )
    }
}

impl From<ProofStatePairingBased> for ProofStatePairingBasedV1 {
    fn from(t: ProofStatePairingBased) -> Self {
        ProofStatePairingBasedV1::new(
            mina_serialization_types::protocol_state_proof::ProofStatePairingBased {
                sg: t.sg.into(),
                old_bulletproof_challenges: t.old_bulletproof_challenges.into(),
            },
        )
    }
}
impl From<ProofStatePairingBasedV1> for ProofStatePairingBased {
    fn from(t: ProofStatePairingBasedV1) -> Self {
        Self {
            sg: t.t.sg.into(),
            old_bulletproof_challenges: t.t.old_bulletproof_challenges.into(),
        }
    }
}

impl From<SpongeDigestBeforeEvaluations> for SpongeDigestBeforeEvaluationsV1 {
    fn from(t: SpongeDigestBeforeEvaluations) -> Self {
        SpongeDigestBeforeEvaluationsV1::new(Versioned::new(
            mina_serialization_types::protocol_state_proof::SpongeDigestBeforeEvaluations (
                (t.0.0.into(), t.0.1.into(), t.0.2.into(), t.0.3.into(), ())
            ),
        ))
    }
}
impl From<SpongeDigestBeforeEvaluationsV1> for SpongeDigestBeforeEvaluations {
    fn from(t: SpongeDigestBeforeEvaluationsV1) -> Self {
        Self (
            (t.t.t.0.0.t.into(), t.t.t.0.1.t.into(), t.t.t.0.2.t.into(), t.t.t.0.3.t.into(), ())
        )
    }
}


impl From<ShiftedValue> for ShiftedValueV1 {
    fn from(t: ShiftedValue) -> Self {
        use mina_serialization_types::protocol_state_proof::ShiftedValue as SV;
        match t {
            ShiftedValue::ShiftedValue(v) => {
                Self::new(SV::ShiftedValue(v.0))
            }
            _ => unimplemented!(),
        }
    }
}
impl From<ShiftedValueV1> for ShiftedValue {`
    fn from(t: ShiftedValueV1) -> Self {
        use mina_serialization_types::protocol_state_proof::ShiftedValue as SV;
        match t.t {
            SV::ShiftedValue(v) => Self::ShiftedValue(v.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<Plonk> for PlonkV1 {
    fn from(t: Plonk) -> Self {
        PlonkV1::new(
            mina_serialization_types::protocol_state_proof::Plonk {
                alpha: t.alpha.into(),
                beta: t.beta.into(),
                gamma: t.gamma.into(),
                zeta: t.zeta.into(),
            },
        )
    }
}
impl From<PlonkV1> for Plonk {
    fn from(t: PlonkV1) -> Self {
        Self {
            alpha: t.t.alpha.into(),
            beta: t.t.beta.into(),
            gamma: t.t.gamma.into(),
            zeta: t.t.zeta.into(),
        }
    }
}

impl From<ProofStateDeferredValues> for ProofStateDeferredValuesV1 {
    fn from(t: ProofStateDeferredValues) -> Self {
        ProofStateDeferredValuesV1::new(
            mina_serialization_types::protocol_state_proof::ProofStateDeferredValues {
                plonk: t.plonk.into(),
                combined_inner_product: t.combined_inner_product.into(),
                b: t.b.into(),
                xi: t.xi.into(),
                bulletproof_challenges: t.bulletproof_challenges.into(),
                which_branch: t.which_branch.into(),
            },
        )
    }
}
impl From<ProofStateDeferredValuesV1> for ProofStateDeferredValues {
    fn from(t: ProofStateDeferredValuesV1) -> Self {
        Self {
            plonk: t.t.plonk.into(),
            combined_inner_product: t.t.combined_inner_product.into(),
            b: t.t.b.into(),
            xi: t.t.xi.into(),
            bulletproof_challenges: t.t.bulletproof_challenges.into(),
            which_branch: t.t.which_branch.into(),
        }
    }
}

impl From<ProofState> for ProofStateV1 {
    fn from(t: ProofState) -> Self {
        ProofStateV1::new(Versioned::new(
            mina_serialization_types::protocol_state_proof::ProofState {
                deferred_values: t.deferred_values.into(),
                sponge_digest_before_evaluations: t.sponge_digest_before_evaluations.into(),
                me_only: t.me_only.into(),
            },
        ))
    }
}
impl From<ProofStateV1> for ProofState {
    fn from(t: ProofStateV1) -> Self {
        Self {
            deferred_values: t.t.deferred_values.into(),
            sponge_digest_before_evaluations: t.t.sponge_digest_before_evaluations.into(),
            me_only: t.t.me_only.into(),
        }
    }
}

impl From<ProofStatement> for ProofStatementV1 {
    fn from(t: ProofStatement) -> Self {
        ProofStatementV1::new(Versioned::new(
            mina_serialization_types::protocol_state_proof::ProofStatement {
                proof_state: t.proof_state.into(),
                pass_through: t.pass_through.into(),
            },
        ))
    }
}
impl From<ProofStatementV1> for ProofStatement {
    fn from(t: ProofStatementV1) -> Self {
        Self {
            proof_state: t.t.t.proof_state.into(),
            pass_through: t.t.t.pass_through.into(),
        }
    }
}

impl From<ProtocolStateProof> for ProtocolStateProofV1 {
    fn from(t: ProtocolStateProof) -> Self {
        ProtocolStateProofV1::new(Versioned::new(
            mina_serialization_types::protocol_state_proof::ProtocolStateProof {
                statement: t.statement.into(),
                prev_evals: t.prev_evals.into(),
                prev_x_hat: t.prev_x_hat.into(),
                proof: t.proof.into(),
            },
        ))
    }
}
impl From<ProtocolStateProofV1> for ProtocolStateProof {
    fn from(t: ProtocolStateProofV1) -> Self {
        Self {
            statement: t.t.t.t.t.statement.into(),
            prev_evals: t.t.t.t.t.prev_evals.into(),
            prev_x_hat: t.t.t.t.t.prev_x_hat.into(),
            proof: t.t.t.t.t.proof.into(),
        }
    }
}