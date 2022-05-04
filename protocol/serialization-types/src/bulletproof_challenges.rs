// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types that capture serialized bullet proof challenges and proofs

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::v1::Hex64V1;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallenge {
    pub prechallenge: BulletproofPreChallengeV1,
}

pub type BulletproofChallengeV1 = Versioned<BulletproofChallenge, 1>;

pub type BulletproofChallengesV1 = Versioned<Vec<BulletproofChallengeTuple18V1>, 1>;

pub type ProofStateBulletproofChallengesV1 = Versioned<
    (
        BulletproofChallengeTuple17V1,
        BulletproofChallengeTuple17V1,
        (),
    ),
    1,
>;

// TODO - see if this can be rewritten with const generics over an array
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallengeTuple17(
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub (),
);

pub type BulletproofChallengeTuple17V1 =
    Versioned<Versioned<Versioned<BulletproofChallengeTuple17, 1>, 1>, 1>;

// TODO - see if this can be rewritten with const generics over an array
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallengeTuple18(
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub BulletproofChallengeV1,
    pub (),
);

pub type BulletproofChallengeTuple18V1 = Versioned<Versioned<BulletproofChallengeTuple18, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2V1),
}

pub type BulletproofPreChallengeV1 = Versioned<BulletproofPreChallenge, 1>;

pub type ScalarChallengeVector2V1 = Versioned<(Hex64V1, Hex64V1, ()), 1>;