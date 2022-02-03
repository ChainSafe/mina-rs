use versioned::Versioned;
use crate::types::*;
use mina_network_types::v1::*;


impl From<ExternalTransition> for ExternalTransitionV1 {
    fn from(t: ExternalTransition) -> Self {
        ExternalTransitionV1(Versioned::new(
            mina_network_types::external_transition::ExternalTransition{
                protocol_state: t.protocol_state.into(),
                protocol_state_proof: t.protocol_state_proof.into(),
                staged_ledger_diff: t.staged_ledger_diff.into(),
                delta_transition_chain_proof: t.delta_transition_chain_proof.into(),
                current_protocol_version: t.current_protocol_version.into(),
                proposed_protocol_version_opt: t.proposed_protocol_version_opt.map(Into::into),
                validation_callback: (),           
            }
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

use mina_network_types::delta_transition_chain_proof::DeltaTransitionChainProof as DeltaTransitionChainProofV1;

impl From<crate::types::DeltaTransitionChainProof> for DeltaTransitionChainProofV1 {
    fn from(t: crate::types::DeltaTransitionChainProof) -> Self {
        Self(t.0.into(), t.1.into())
    }
}
impl From<DeltaTransitionChainProofV1> for crate::types::DeltaTransitionChainProof {
    fn from(t: DeltaTransitionChainProofV1) -> Self {
        Self(t.0.into(), t.1.iter().map(Into::into).collect())
    }
}

impl From<ProtocolVersion> for ProtocolVersionV1 {
    fn from(t: ProtocolVersion) -> Self {
        ProtocolVersionV1::new(    
            mina_network_types::protocol_version::ProtocolVersion {
            	major: t.major,
            	minor: t.minor,
            	patch: t.patch,
            }
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
