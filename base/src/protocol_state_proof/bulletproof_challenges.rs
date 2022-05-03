// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::Hex64;
use mina_serialization_types_macros::AutoFrom;

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::BulletproofChallenge)]
pub struct BulletproofChallenge {
    pub prechallenge: BulletproofPreChallenge,
}

impl BulletproofChallenge {
    pub fn new(a: i64, b: i64) -> Self {
        Self {
            prechallenge: BulletproofPreChallenge::scalar(a, b),
        }
    }
}

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::BulletproofChallenges)]
pub struct BulletproofChallenges(pub Vec<BulletproofChallengeTuple18>);

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::ProofStateBulletproofChallenges)]
pub struct ProofStateBulletproofChallenges(
    pub BulletproofChallengeTuple17,
    pub BulletproofChallengeTuple17,
    pub (),
);

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::BulletproofChallengeTuple17)]
pub struct BulletproofChallengeTuple17(
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub (),
);

impl BulletproofChallengeTuple17 {
    pub fn new(a: [i64; 34]) -> Self {
        Self(
            BulletproofChallenge::new(a[0], a[1]),
            BulletproofChallenge::new(a[2], a[3]),
            BulletproofChallenge::new(a[4], a[5]),
            BulletproofChallenge::new(a[6], a[7]),
            BulletproofChallenge::new(a[8], a[9]),
            BulletproofChallenge::new(a[10], a[11]),
            BulletproofChallenge::new(a[12], a[13]),
            BulletproofChallenge::new(a[14], a[15]),
            BulletproofChallenge::new(a[16], a[17]),
            BulletproofChallenge::new(a[18], a[19]),
            BulletproofChallenge::new(a[20], a[21]),
            BulletproofChallenge::new(a[22], a[23]),
            BulletproofChallenge::new(a[24], a[25]),
            BulletproofChallenge::new(a[26], a[27]),
            BulletproofChallenge::new(a[28], a[29]),
            BulletproofChallenge::new(a[30], a[31]),
            BulletproofChallenge::new(a[32], a[33]),
            (),
        )
    }
}

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::BulletproofChallengeTuple18)]
pub struct BulletproofChallengeTuple18(
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub BulletproofChallenge,
    pub (),
);

impl BulletproofChallengeTuple18 {
    pub fn new(a: [i64; 36]) -> Self {
        Self(
            BulletproofChallenge::new(a[0], a[1]),
            BulletproofChallenge::new(a[2], a[3]),
            BulletproofChallenge::new(a[4], a[5]),
            BulletproofChallenge::new(a[6], a[7]),
            BulletproofChallenge::new(a[8], a[9]),
            BulletproofChallenge::new(a[10], a[11]),
            BulletproofChallenge::new(a[12], a[13]),
            BulletproofChallenge::new(a[14], a[15]),
            BulletproofChallenge::new(a[16], a[17]),
            BulletproofChallenge::new(a[18], a[19]),
            BulletproofChallenge::new(a[20], a[21]),
            BulletproofChallenge::new(a[22], a[23]),
            BulletproofChallenge::new(a[24], a[25]),
            BulletproofChallenge::new(a[26], a[27]),
            BulletproofChallenge::new(a[28], a[29]),
            BulletproofChallenge::new(a[30], a[31]),
            BulletproofChallenge::new(a[32], a[33]),
            BulletproofChallenge::new(a[34], a[35]),
            (),
        )
    }
}

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::BulletproofPreChallenge)]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2),
}

impl BulletproofPreChallenge {
    pub fn scalar(a: i64, b: i64) -> Self {
        Self::ScalarChallenge(ScalarChallengeVector2::new(a, b))
    }
}

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::bulletproof_challenges::ScalarChallengeVector2)]
pub struct ScalarChallengeVector2(pub Hex64, pub Hex64, pub ());

impl ScalarChallengeVector2 {
    pub fn new(a: i64, b: i64) -> Self {
        Self(a.into(), b.into(), ())
    }
}

impl Default for BulletproofPreChallenge {
    fn default() -> Self {
        Self::ScalarChallenge(ScalarChallengeVector2::default())
    }
}
