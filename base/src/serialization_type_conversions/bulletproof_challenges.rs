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

impl_from_for_versioned_with_proxy!(
    ProofStateBulletproofChallenges,
    mina_serialization_types::bulletproof_challenges::ProofStateBulletproofChallenges,
    ProofStateBulletproofChallengesV1
);

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

impl_from_for_versioned_with_proxy!(
    ScalarChallengeVector2,
    mina_serialization_types::bulletproof_challenges::ScalarChallengeVector2,
    ScalarChallengeVector2V1
);
