// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::v1::*;
use versioned::*;

impl_from_for_versioned_with_proxy!(
    BulletproofChallenge,
    mina_serialization_types::bulletproof_challenges::BulletproofChallenge,
    BulletproofChallengeV1
);

impl From<BulletproofChallenges> for BulletproofChallengesV1 {
    fn from(t: BulletproofChallenges) -> Self {
        t.0.into_iter().map(Into::into).collect::<Vec<_>>().into()
    }
}
impl From<BulletproofChallengesV1> for BulletproofChallenges {
    fn from(t: BulletproofChallengesV1) -> Self {
        Self(t.t.into_iter().map(Into::into).collect())
    }
}

impl From<ProofStateBulletproofChallenges> for ProofStateBulletproofChallengesV1 {
    fn from(t: ProofStateBulletproofChallenges) -> Self {
        (t.0 .0.into(), t.0 .1.into(), ()).into()
    }
}
impl From<ProofStateBulletproofChallengesV1> for ProofStateBulletproofChallenges {
    fn from(t: ProofStateBulletproofChallengesV1) -> Self {
        Self((t.t.0.into(), t.t.1.into(), ()))
    }
}

impl_from_for_versioned_with_proxy!(
    BulletproofChallengeTuple17,
    mina_serialization_types::bulletproof_challenges::BulletproofChallengeTuple17,
    BulletproofChallengeTuple17V1
);

impl_from_for_versioned_with_proxy!(
    BulletproofChallengeTuple18,
    mina_serialization_types::bulletproof_challenges::BulletproofChallengeTuple18,
    BulletproofChallengeTuple18V1
);

impl From<BulletproofPreChallenge> for BulletproofPreChallengeV1 {
    fn from(t: BulletproofPreChallenge) -> Self {
        use mina_serialization_types::bulletproof_challenges::BulletproofPreChallenge as PC;
        match t {
            BulletproofPreChallenge::ScalarChallenge(v) => PC::ScalarChallenge(v.into()).into(),
        }
    }
}
impl From<BulletproofPreChallengeV1> for BulletproofPreChallenge {
    fn from(t: BulletproofPreChallengeV1) -> Self {
        use mina_serialization_types::bulletproof_challenges::BulletproofPreChallenge as PC;
        match t.t {
            PC::ScalarChallenge(v) => Self::ScalarChallenge(v.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<ScalarChallengeVector2> for ScalarChallengeVector2V1 {
    fn from(t: ScalarChallengeVector2) -> Self {
        (t.0 .0.into(), t.0 .1.into(), ()).into()
    }
}
impl From<ScalarChallengeVector2V1> for ScalarChallengeVector2 {
    fn from(t: ScalarChallengeVector2V1) -> Self {
        Self((t.t.0.t.into(), t.t.1.t.into(), ()))
    }
}
