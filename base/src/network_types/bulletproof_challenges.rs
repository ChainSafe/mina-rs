// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::types::Hex64;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct BulletproofChallenge {
    pub prechallenge: BulletproofPreChallengeV1,
}

pub type BulletproofChallengeV1 = Versioned<BulletproofChallenge, 1>;

pub type BulletproofChallengesV1 = Versioned<Vec<BulletproofChallengeTuple18V1>, 1>;

pub type ProofStateBulletproofChallengesV1 = Versioned<(BulletproofChallengeTuple17V1, BulletproofChallengeTuple17V1, ()), 1>;

// TODO - see if this can be rewritten with const generics over an array
#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
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

pub type BulletproofChallengeTuple17V1 = Versioned<Versioned<Versioned<BulletproofChallengeTuple17, 1>, 1>, 1>;

// TODO - see if this can be rewritten with const generics over an array
#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
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


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2V1),
}

impl Default for BulletproofPreChallenge {
    fn default() -> BulletproofPreChallenge {
        Self::ScalarChallenge(Default::default())
    }
}

pub type BulletproofPreChallengeV1 = Versioned<BulletproofPreChallenge, 1>;


pub type ScalarChallengeVector2V1 = Versioned<(Hex64, Hex64, ()), 1>;
