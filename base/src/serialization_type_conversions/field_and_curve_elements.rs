// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::FiniteECPoint;
use crate::types::*;
use mina_serialization_types::v1::FiniteECPoint as FiniteECPointV1;
use mina_serialization_types::v1::*;

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

impl From<ECPointVec> for ECPointVecV1 {
    fn from(t: ECPointVec) -> Self {
        t.0.into_iter().map(Into::into).collect::<Vec<_>>().into()
    }
}
impl From<ECPointVecV1> for ECPointVec {
    fn from(t: ECPointVecV1) -> Self {
        Self(t.t.into_iter().map(Into::into).collect())
    }
}

impl From<FiniteECPointVec> for FiniteECPointVecV1 {
    fn from(t: FiniteECPointVec) -> Self {
        t.0.into_iter().map(Into::into).collect::<Vec<_>>().into()
    }
}
impl From<FiniteECPointVecV1> for FiniteECPointVec {
    fn from(t: FiniteECPointVecV1) -> Self {
        Self(t.t.into_iter().map(Into::into).collect())
    }
}

impl From<FiniteECPoint> for FiniteECPointV1 {
    fn from(t: FiniteECPoint) -> Self {
        Self(t.0 .0, t.1 .0)
    }
}
impl From<FiniteECPointV1> for FiniteECPoint {
    fn from(t: FiniteECPointV1) -> Self {
        Self(t.0.into(), t.1.into())
    }
}

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
