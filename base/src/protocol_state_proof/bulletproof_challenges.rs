use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::types::Hex64;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BulletproofChallenge {
    pub prechallenge: BulletproofPreChallenge,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BulletproofChallenges(Vec<BulletproofChallengeTuple18>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofStateBulletproofChallenges(
    (BulletproofChallengeTuple17, BulletproofChallengeTuple17, ()),
);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
pub struct BulletproofChallengeTuple17(
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    (),
);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct BulletproofChallengeTuple18(
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    (),
);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2),
}

#[derive(Clone, Serialize, Default, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ScalarChallengeVector2((Hex64, Hex64, ()));

impl Default for BulletproofPreChallenge {
    fn default() -> Self {
        Self::ScalarChallenge(ScalarChallengeVector2::default())
    }
}
