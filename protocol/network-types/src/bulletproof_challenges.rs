// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types that capture serialized bullet proof challenges and proofs

use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::v1::Hex64V1;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallengeTuple17(
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    (),
);

pub type BulletproofChallengeTuple17V1 =
    Versioned<Versioned<Versioned<BulletproofChallengeTuple17, 1>, 1>, 1>;

// TODO - see if this can be rewritten with const generics over an array
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallengeTuple18(
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    BulletproofChallengeV1,
    (),
);

pub type BulletproofChallengeTuple18V1 = Versioned<Versioned<BulletproofChallengeTuple18, 1>, 1>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2V1),
}

pub type BulletproofPreChallengeV1 = Versioned<BulletproofPreChallenge, 1>;

pub type ScalarChallengeVector2V1 = Versioned<(Hex64V1, Hex64V1, ()), 1>;
