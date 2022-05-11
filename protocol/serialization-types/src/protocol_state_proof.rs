// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module containing the components of a protocol state proof

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::field_and_curve_elements::FiniteECPoint;
use crate::{common::*, json::*, v1::*, *};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use versioned::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// SNARK proof of the protocol state at some point in time
pub struct ProtocolStateProof {
    pub statement: ProofStatementV1,
    pub prev_evals: PrevEvalsV1,
    pub prev_x_hat: PrevXHatV1,
    pub proof: ProofV1,
}

/// SNARK proof of the protocol state at some point in time (v1)
pub type ProtocolStateProofV1 =
    Versioned<Versioned<Versioned<Versioned<ProtocolStateProof, 1>, 1>, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProtocolStateProof)]
/// SNARK proof of the protocol state at some point in time (json)
pub struct ProtocolStateProofJson {
    pub statement: ProofStatementJson,
    pub prev_evals: PrevEvalsJson,
    pub prev_x_hat: PrevXHatJson,
    pub proof: ProofJson,
}

/// SNARK proof of the protocol state at some point in time (base64 json)
#[derive(Clone, Debug, PartialEq, derive_more::From, derive_more::Into)]
pub struct ProtocolStateProofBase64Json(pub ProtocolStateProofV1);

impl Serialize for ProtocolStateProofBase64Json {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes: Vec<u8> = Vec::new();
        bin_prot::to_writer(&mut bytes, &self.0)
            .map_err(<S::Error as serde::ser::Error>::custom)?;
        let s = base64::encode_config(bytes, base64::URL_SAFE);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ProtocolStateProofBase64Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = base64::decode_config(s, base64::URL_SAFE)
            .map_err(<D::Error as serde::de::Error>::custom)?;
        let t: ProtocolStateProofV1 = bin_prot::from_reader(bytes.as_slice())
            .map_err(<D::Error as serde::de::Error>::custom)?;
        Ok(t.into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStatement {
    pub proof_state: ProofStateV1,
    pub pass_through: PairingBasedV1,
}

pub type ProofStatementV1 = Versioned<Versioned<ProofStatement, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofStatement)]
pub struct ProofStatementJson {
    pub proof_state: ProofStateJson,
    pub pass_through: PairingBasedJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofState {
    pub deferred_values: ProofStateDeferredValuesV1,
    pub sponge_digest_before_evaluations: SpongeDigestBeforeEvaluationsV1,
    pub me_only: ProofStatePairingBasedV1,
}

pub type ProofStateV1 = Versioned<ProofState, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofState)]
pub struct ProofStateJson {
    pub deferred_values: ProofStateDeferredValuesJson,
    pub sponge_digest_before_evaluations: SpongeDigestBeforeEvaluationsJson,
    pub me_only: ProofStatePairingBasedJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStateDeferredValues {
    pub plonk: PlonkV1,
    pub combined_inner_product: ShiftedValueV1,
    pub b: ShiftedValueV1,
    pub xi: BulletproofPreChallengeV1,
    pub bulletproof_challenges: BulletproofChallengeTuple18V1,
    pub which_branch: CharV1,
}

pub type ProofStateDeferredValuesV1 = Versioned<ProofStateDeferredValues, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofStateDeferredValues)]
pub struct ProofStateDeferredValuesJson {
    pub plonk: PlonkJson,
    pub combined_inner_product: ShiftedValueMinaJson,
    pub b: ShiftedValueMinaJson,
    pub xi: BulletproofPreChallengeMinaJson,
    pub bulletproof_challenges: BulletproofChallengeTuple18Json,
    pub which_branch: CharJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Plonk {
    pub alpha: BulletproofPreChallengeV1,
    pub beta: ScalarChallengeVector2V1,
    pub gamma: ScalarChallengeVector2V1,
    pub zeta: BulletproofPreChallengeV1,
}

pub type PlonkV1 = Versioned<Plonk, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(Plonk)]
pub struct PlonkJson {
    pub alpha: BulletproofPreChallengeMinaJson,
    pub beta: ScalarChallengeVector2Json,
    pub gamma: ScalarChallengeVector2Json,
    pub zeta: BulletproofPreChallengeMinaJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ShiftedValue {
    ShiftedValue(BigInt256),
}

pub type ShiftedValueV1 = Versioned<ShiftedValue, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum ShiftedValueJson {
    #[serde(rename = "Shifted_value")]
    ShiftedValue(FieldElementJson),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(ShiftedValue)]
#[auto_from(ShiftedValueJson)]
pub enum ShiftedValueMinaJson {
    ShiftedValue(FieldElementJson),
}
impl_mina_enum_json_serde!(ShiftedValueMinaJson, ShiftedValueJson);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SpongeDigestBeforeEvaluations(
    pub Hex64V1,
    pub Hex64V1,
    pub Hex64V1,
    pub Hex64V1,
    pub (),
);

pub type SpongeDigestBeforeEvaluationsV1 =
    Versioned<Versioned<SpongeDigestBeforeEvaluations, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(SpongeDigestBeforeEvaluations)]
pub struct SpongeDigestBeforeEvaluationsJson(
    pub I64,
    pub I64,
    pub I64,
    pub I64,
    #[serde(skip)] pub (),
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStatePairingBased {
    pub sg: FiniteECPoint,
    pub old_bulletproof_challenges: ProofStateBulletproofChallengesV1,
}

pub type ProofStatePairingBasedV1 = Versioned<ProofStatePairingBased, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofStatePairingBased)]
pub struct ProofStatePairingBasedJson {
    pub sg: FiniteECPointJson,
    pub old_bulletproof_challenges: ProofStateBulletproofChallengesJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PairingBased {
    pub app_state: (),
    pub sg: FiniteECPointVecV1,
    pub old_bulletproof_challenges: BulletproofChallengesV1,
}

pub type PairingBasedV1 = Versioned<PairingBased, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(PairingBased)]
pub struct PairingBasedJson {
    pub app_state: (),
    pub sg: FiniteECPointVecJson,
    pub old_bulletproof_challenges: BulletproofChallengesJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrevEvals(pub ProofEvaluationsV1, pub ProofEvaluationsV1);

pub type PrevEvalsV1 = Versioned<PrevEvals, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(PrevEvals)]
pub struct PrevEvalsJson(pub ProofEvaluationsV1, pub ProofEvaluationsV1);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrevXHat(pub FiniteECPoint);

pub type PrevXHatV1 = Versioned<PrevXHat, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(PrevXHat)]
pub struct PrevXHatJson(pub FiniteECPoint);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Proof {
    pub messages: ProofMessagesV1,
    pub openings: ProofOpeningsV1,
}

pub type ProofV1 = Versioned<Versioned<Proof, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(Proof)]
pub struct ProofJson {
    pub messages: ProofMessagesJson,
    pub openings: ProofOpeningsJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofOpeningsEvals(pub ProofEvaluationsV1, pub ProofEvaluationsV1);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofOpeningsEvals)]
pub struct ProofOpeningsEvalsJson(pub ProofEvaluationsJson, pub ProofEvaluationsJson);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofOpenings {
    pub proof: OpeningProofV1,
    pub evals: ProofOpeningsEvals,
}

pub type ProofOpeningsV1 = Versioned<ProofOpenings, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofOpenings)]
pub struct ProofOpeningsJson {
    pub proof: OpeningProofJson,
    pub evals: ProofOpeningsEvalsJson,
}
