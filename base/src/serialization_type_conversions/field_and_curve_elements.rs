// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::v1::*;
use versioned::*;

impl From<FieldElementVec> for FieldElementVecV1 {
    fn from(t: FieldElementVec) -> Self {
        t.0.into_iter().map(|v| v.0).collect::<Vec<_>>().into()
    }
}
impl From<FieldElementVecV1> for FieldElementVec {
    fn from(t: FieldElementVecV1) -> Self {
        Self(t.t.into_iter().map(Into::into).collect())
    }
}

impl From<FiniteECPointPairVec> for FiniteECPointPairVecV1 {
    fn from(t: FiniteECPointPairVec) -> Self {
        t.0.into_iter()
            .map(|(v1, v2)| (v1.into(), v2.into()))
            .collect::<Vec<_>>()
            .into()
    }
}
impl From<FiniteECPointPairVecV1> for FiniteECPointPairVec {
    fn from(t: FiniteECPointPairVecV1) -> Self {
        Self(
            t.t.into_iter()
                .map(|(v1, v2)| (v1.into(), v2.into()))
                .collect(),
        )
    }
}

impl_from_for_versioned_with_proxy!(
    ECPointVec,
    mina_serialization_types::field_and_curve_elements::ECPointVec,
    ECPointVecV1
);

impl_from_for_versioned_with_proxy!(
    FiniteECPointVec,
    mina_serialization_types::field_and_curve_elements::FiniteECPointVec,
    FiniteECPointVecV1
);

impl From<ECPoint> for ECPointV1 {
    fn from(t: ECPoint) -> Self {
        use mina_serialization_types::field_and_curve_elements::ECPoint as EC;
        match t {
            ECPoint::Infinite => EC::Infinite,
            ECPoint::Finite(v) => EC::Finite(v.into()),
        }
        .into()
    }
}
impl From<ECPointV1> for ECPoint {
    fn from(t: ECPointV1) -> Self {
        use mina_serialization_types::field_and_curve_elements::ECPoint as EC;
        match t.t {
            EC::Infinite => Self::Infinite,
            EC::Finite(v) => Self::Finite(v.into()),
        }
    }
}
