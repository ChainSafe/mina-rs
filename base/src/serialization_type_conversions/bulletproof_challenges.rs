// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::v1::*;

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
