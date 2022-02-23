use crate::types::proof_messages::{
    ProofMessageWithDegreeBound, ProofMessageWithoutDegreeBoundList,
};
use crate::types::*;
use mina_serialization_types::v1::*;
use versioned::Versioned;

impl From<ProofMessageWithDegreeBound> for ProofMessageWithDegreeBoundV1 {
    fn from(t: ProofMessageWithDegreeBound) -> Self {
        ProofMessageWithDegreeBoundV1::new(
            mina_serialization_types::proof_messages::ProofMessageWithDegreeBound {
                unshifted: t.unshifted.into(),
                shifted: t.shifted.into(),
            },
        )
    }
}
impl From<ProofMessageWithDegreeBoundV1> for ProofMessageWithDegreeBound {
    fn from(t: ProofMessageWithDegreeBoundV1) -> Self {
        Self {
            unshifted: t.t.unshifted.into(),
            shifted: t.t.shifted.into(),
        }
    }
}

impl From<ProofMessageWithoutDegreeBoundList> for ProofMessageWithoutDegreeBoundListV1 {
    fn from(t: ProofMessageWithoutDegreeBoundList) -> Self {
        ProofMessageWithoutDegreeBoundListV1::new(Versioned::new(
            t.0.into_iter().map(Into::into).collect(),
        ))
    }
}
impl From<ProofMessageWithoutDegreeBoundListV1> for ProofMessageWithoutDegreeBoundList {
    fn from(t: ProofMessageWithoutDegreeBoundListV1) -> Self {
        Self(t.t.t.into_iter().map(Into::into).collect())
    }
}

impl From<ProofMessages> for ProofMessagesV1 {
    fn from(t: ProofMessages) -> Self {
        ProofMessagesV1::new(mina_serialization_types::proof_messages::ProofMessages {
            l_comm: t.l_comm.into(),
            r_comm: t.r_comm.into(),
            o_comm: t.o_comm.into(),
            z_comm: t.z_comm.into(),
            t_comm: t.t_comm.into(),
        })
    }
}
impl From<ProofMessagesV1> for ProofMessages {
    fn from(t: ProofMessagesV1) -> Self {
        Self {
            l_comm: t.t.l_comm.into(),
            r_comm: t.t.r_comm.into(),
            o_comm: t.t.o_comm.into(),
            z_comm: t.t.z_comm.into(),
            t_comm: t.t.t_comm.into(),
        }
    }
}

impl From<ProofEvaluations> for ProofEvaluationsV1 {
    fn from(t: ProofEvaluations) -> Self {
        ProofEvaluationsV1::new(
            mina_serialization_types::proof_evaluations::ProofEvaluations {
                l: t.l.into(),
                r: t.r.into(),
                o: t.o.into(),
                z: t.z.into(),
                t: t.t.into(),
                f: t.f.into(),
                sigma1: t.sigma1.into(),
                sigma2: t.sigma2.into(),
            },
        )
    }
}
impl From<ProofEvaluationsV1> for ProofEvaluations {
    fn from(t: ProofEvaluationsV1) -> Self {
        Self {
            l: t.t.l.into(),
            r: t.t.r.into(),
            o: t.t.o.into(),
            z: t.t.z.into(),
            t: t.t.t.into(),
            f: t.t.f.into(),
            sigma1: t.t.sigma1.into(),
            sigma2: t.t.sigma2.into(),
        }
    }
}

impl From<OpeningProof> for OpeningProofV1 {
    fn from(t: OpeningProof) -> Self {
        OpeningProofV1::new(mina_serialization_types::opening_proof::OpeningProof {
            lr: t.lr.into(),
            z_1: t.z_1.0.into(),
            z_2: t.z_2.0.into(),
            delta: t.delta.into(),
            sg: t.sg.into(),
        })
    }
}
impl From<OpeningProofV1> for OpeningProof {
    fn from(t: OpeningProofV1) -> Self {
        Self {
            lr: t.t.lr.into(),
            z_1: t.t.z_1.into(),
            z_2: t.t.z_2.into(),
            delta: t.t.delta.into(),
            sg: t.t.sg.into(),
        }
    }
}

impl From<ProofOpenings> for ProofOpeningsV1 {
    fn from(t: ProofOpenings) -> Self {
        ProofOpeningsV1::new(
            mina_serialization_types::protocol_state_proof::ProofOpenings {
                proof: t.proof.into(),
                evals: (t.evals.0.into(), t.evals.1.into()),
            },
        )
    }
}
impl From<ProofOpeningsV1> for ProofOpenings {
    fn from(t: ProofOpeningsV1) -> Self {
        Self {
            proof: t.t.proof.into(),
            evals: (t.t.evals.0.into(), t.t.evals.1.into()),
        }
    }
}

impl From<Proof> for ProofV1 {
    fn from(t: Proof) -> Self {
        ProofV1::new(Versioned::new(
            mina_serialization_types::protocol_state_proof::Proof {
                messages: t.messages.into(),
                openings: t.openings.into(),
            },
        ))
    }
}
impl From<ProofV1> for Proof {
    fn from(t: ProofV1) -> Self {
        Self {
            messages: t.t.t.messages.into(),
            openings: t.t.t.openings.into(),
        }
    }
}

impl From<PrevXHat> for PrevXHatV1 {
    fn from(t: PrevXHat) -> Self {
        PrevXHatV1::new(t.0.into())
    }
}
impl From<PrevXHatV1> for PrevXHat {
    fn from(t: PrevXHatV1) -> Self {
        Self(t.t.into())
    }
}

impl From<PrevEvals> for PrevEvalsV1 {
    fn from(t: PrevEvals) -> Self {
        PrevEvalsV1::new((t.0 .0.into(), t.0 .1.into()))
    }
}
impl From<PrevEvalsV1> for PrevEvals {
    fn from(t: PrevEvalsV1) -> Self {
        Self((t.t.0.into(), t.t.1.into()))
    }
}

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
            mina_serialization_types::protocol_state_proof::SpongeDigestBeforeEvaluations((
                t.0 .0.into(),
                t.0 .1.into(),
                t.0 .2.into(),
                t.0 .3.into(),
                (),
            )),
        ))
    }
}
impl From<SpongeDigestBeforeEvaluationsV1> for SpongeDigestBeforeEvaluations {
    fn from(t: SpongeDigestBeforeEvaluationsV1) -> Self {
        Self((
            t.t.t.0 .0.t.into(),
            t.t.t.0 .1.t.into(),
            t.t.t.0 .2.t.into(),
            t.t.t.0 .3.t.into(),
            (),
        ))
    }
}

impl From<ShiftedValue> for ShiftedValueV1 {
    fn from(t: ShiftedValue) -> Self {
        use mina_serialization_types::protocol_state_proof::ShiftedValue as SV;
        match t {
            ShiftedValue::ShiftedValue(v) => Self::new(SV::ShiftedValue(v.0)),
        }
    }
}
impl From<ShiftedValueV1> for ShiftedValue {
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
        PlonkV1::new(mina_serialization_types::protocol_state_proof::Plonk {
            alpha: t.alpha.into(),
            beta: t.beta.into(),
            gamma: t.gamma.into(),
            zeta: t.zeta.into(),
        })
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
                which_branch: t.which_branch.0.into(),
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
            which_branch: t.t.which_branch.t.into(),
        }
    }
}

impl From<ProofState> for ProofStateV1 {
    fn from(t: ProofState) -> Self {
        ProofStateV1::new(mina_serialization_types::protocol_state_proof::ProofState {
            deferred_values: t.deferred_values.into(),
            sponge_digest_before_evaluations: t.sponge_digest_before_evaluations.into(),
            me_only: t.me_only.into(),
        })
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
        ProtocolStateProofV1::new(Versioned::new(Versioned::new(Versioned::new(
            mina_serialization_types::protocol_state_proof::ProtocolStateProof {
                statement: t.statement.into(),
                prev_evals: t.prev_evals.into(),
                prev_x_hat: t.prev_x_hat.into(),
                proof: t.proof.into(),
            },
        ))))
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
