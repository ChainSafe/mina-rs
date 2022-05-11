// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types that capture serialized bullet proof challenges and proofs

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::{common::*, *};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Serialize};
use versioned::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallenge {
    pub prechallenge: BulletproofPreChallengeV1,
}

pub type BulletproofChallengeV1 = Versioned<BulletproofChallenge, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(BulletproofChallenge)]
pub struct BulletproofChallengeJson {
    pub prechallenge: BulletproofPreChallengeMinaJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulletproofChallenges(pub Vec<BulletproofChallengeTuple18V1>);

pub type BulletproofChallengesV1 = Versioned<BulletproofChallenges, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(BulletproofChallenges)]
pub struct BulletproofChallengesJson(pub Vec<BulletproofChallengeTuple18Json>);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStateBulletproofChallenges(
    pub BulletproofChallengeTuple17V1,
    pub BulletproofChallengeTuple17V1,
    pub (),
);

pub type ProofStateBulletproofChallengesV1 = Versioned<ProofStateBulletproofChallenges, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ProofStateBulletproofChallenges)]
pub struct ProofStateBulletproofChallengesJson(
    pub BulletproofChallengeTuple17Json,
    pub BulletproofChallengeTuple17Json,
    #[serde(skip)] pub (),
);

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(BulletproofChallengeTuple17)]
pub struct BulletproofChallengeTuple17Json(
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    #[serde(skip)] pub (),
);

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(BulletproofChallengeTuple18)]
pub struct BulletproofChallengeTuple18Json(
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    pub BulletproofChallengeJson,
    #[serde(skip)] pub (),
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2V1),
}

pub type BulletproofPreChallengeV1 = Versioned<BulletproofPreChallenge, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum BulletproofPreChallengeJson {
    #[serde(rename = "Scalar_challenge")]
    ScalarChallenge(ScalarChallengeVector2Json),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(BulletproofPreChallenge)]
#[auto_from(BulletproofPreChallengeJson)]
pub enum BulletproofPreChallengeMinaJson {
    ScalarChallenge(ScalarChallengeVector2Json),
}
impl_mina_enum_json_serde_with_option!(
    BulletproofPreChallengeMinaJson,
    BulletproofPreChallengeJson,
    false
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ScalarChallengeVector2(pub Hex64V1, pub Hex64V1, pub ());

pub type ScalarChallengeVector2V1 = Versioned<ScalarChallengeVector2, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(ScalarChallengeVector2)]
pub struct ScalarChallengeVector2Json(pub I64, pub I64, #[serde(skip)] pub ());
