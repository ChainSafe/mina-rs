// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::hash::BackendCommonHash;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 4)]
pub struct ProtocolStateProof {
    pub statement: bin_prot::Value,
    pub prev_evals: bin_prot::Value,
    pub prev_x_hat: bin_prot::Value,
    pub proof: Proof,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct Proof {
    pub messages: ProofMessages,
    pub openings: ProofOpenings,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessages {
    pub l_comm: ProofMessageWithoutDegreeBoundList,
    pub r_comm: ProofMessageWithoutDegreeBoundList,
    pub o_comm: ProofMessageWithoutDegreeBoundList,
    pub z_comm: ProofMessageWithoutDegreeBoundList,
    pub t_comm: ProofMessageWithDegreeBound,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ProofMessageWithoutDegreeBoundList(Vec<ProofMessageWithoutDegreeBound>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofMessageWithoutDegreeBound((BackendCommonHash, BackendCommonHash));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessageWithDegreeBound {
    pub unshifted: ProofMessageWithDegreeBoundFiniteOrInfiniteList,
    pub shifted: ProofMessageWithDegreeBoundFiniteOrInfinite,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessageWithDegreeBoundFiniteOrInfiniteList(
    Vec<ProofMessageWithDegreeBoundFiniteOrInfinite>,
);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub enum ProofMessageWithDegreeBoundFiniteOrInfinite {
    Infinite,
    Finite(ProofMessageWithoutDegreeBound),
}

impl Default for ProofMessageWithDegreeBoundFiniteOrInfinite {
    fn default() -> Self {
        Self::Infinite
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpenings {
    pub proof: ProofOpeningsProof,
    pub evals: ProofOpeningsEvalTuple,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpeningsProof {
    pub lr: BackendCommonHashTupleList,
    pub z_1: BackendCommonHash,
    pub z_2: BackendCommonHash,
    pub delta: BackendCommonHashTuple,
    pub sg: BackendCommonHashTuple,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpeningsEval {
    pub l: ProofOpeningsEvalMessageList,
    pub r: ProofOpeningsEvalMessageList,
    pub o: ProofOpeningsEvalMessageList,
    pub z: ProofOpeningsEvalMessageList,
    pub t: ProofOpeningsEvalMessageList,
    pub f: ProofOpeningsEvalMessageList,
    pub sigma1: ProofOpeningsEvalMessageList,
    pub sigma2: ProofOpeningsEvalMessageList,
}

pub type ProofOpeningsEvalTuple = (ProofOpeningsEval, ProofOpeningsEval);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpeningsEvalMessageList(Vec<BackendCommonHash>);

pub type BackendCommonHashTuple = (BackendCommonHash, BackendCommonHash);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BackendCommonHashTupleList(Vec<(BackendCommonHashTuple, BackendCommonHashTuple)>);
