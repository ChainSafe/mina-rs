use crate::types::*;
use mina_serialization_types::v1::*;
use versioned::Versioned;

impl From<BulletproofChallenge> for BulletproofChallengeV1 {
    fn from(t: BulletproofChallenge) -> Self {
        BulletproofChallengeV1::new(
            mina_serialization_types::bulletproof_challenges::BulletproofChallenge {
                prechallenge: t.prechallenge.into(),
            },
        )
    }
}
impl From<BulletproofChallengeV1> for BulletproofChallenge {
    fn from(t: BulletproofChallengeV1) -> Self {
        Self {
            prechallenge: t.t.prechallenge.into(),
        }
    }
}

impl From<BulletproofChallenges> for BulletproofChallengesV1 {
    fn from(t: BulletproofChallenges) -> Self {
        Versioned::new(t.0.into_iter().map(Into::into).collect())
    }
}
impl From<BulletproofChallengesV1> for BulletproofChallenges {
    fn from(t: BulletproofChallengesV1) -> Self {
        Self(t.t.into_iter().map(Into::into).collect())
    }
}

impl From<ProofStateBulletproofChallenges> for ProofStateBulletproofChallengesV1 {
    fn from(t: ProofStateBulletproofChallenges) -> Self {
    ProofStateBulletproofChallengesV1::new(
            (t.0.0.into(), t.0.1.into(), ())
        )
    }
}
impl From<ProofStateBulletproofChallengesV1> for ProofStateBulletproofChallenges {
    fn from(t: ProofStateBulletproofChallengesV1) -> Self {
        Self (
            (t.t.0.into(), t.t.1.into(), ())
        )
    }
}

impl From<BulletproofChallengeTuple18> for BulletproofChallengeTuple18V1 {
    fn from(t: BulletproofChallengeTuple18) -> Self {
    BulletproofChallengeTuple18V1::new(Versioned::new(
            mina_serialization_types::bulletproof_challenges::BulletproofChallengeTuple18(
            	t.0.into(), t.1.into(), t.2.into(), t.3.into(),
            	t.4.into(), t.5.into(), t.6.into(), t.7.into(),
            	t.8.into(), t.9.into(), t.10.into(), t.11.into(),
            	t.12.into(), t.13.into(), t.14.into(), t.15.into(),
            	t.16.into(), t.17.into(), (), 
            ))
        )
    }
}
impl From<BulletproofChallengeTuple18V1> for BulletproofChallengeTuple18 {
    fn from(t: BulletproofChallengeTuple18V1) -> Self {
        Self (
        	t.t.t.0.into(), t.t.t.1.into(), t.t.t.2.into(), t.t.t.3.into(),
        	t.t.t.4.into(), t.t.t.5.into(), t.t.t.6.into(), t.t.t.7.into(),
        	t.t.t.8.into(), t.t.t.9.into(), t.t.t.10.into(), t.t.t.11.into(),
        	t.t.t.12.into(), t.t.t.13.into(), t.t.t.14.into(), t.t.t.15.into(),
        	t.t.t.16.into(), t.t.t.17.into(), (),
        )
    }
}

impl From<BulletproofChallengeTuple17> for BulletproofChallengeTuple17V1 {
    fn from(t: BulletproofChallengeTuple17) -> Self {
    BulletproofChallengeTuple17V1::new(Versioned::new(Versioned::new(
            mina_serialization_types::bulletproof_challenges::BulletproofChallengeTuple17(
            	t.0.into(), t.1.into(), t.2.into(), t.3.into(),
            	t.4.into(), t.5.into(), t.6.into(), t.7.into(),
            	t.8.into(), t.9.into(), t.10.into(), t.11.into(),
            	t.12.into(), t.13.into(), t.14.into(), t.15.into(),
            	t.16.into(), (), 
            )))
        )
    }
}
impl From<BulletproofChallengeTuple17V1> for BulletproofChallengeTuple17 {
    fn from(t: BulletproofChallengeTuple17V1) -> Self {
        Self (
        	t.t.t.t.0.into(), t.t.t.t.1.into(), t.t.t.t.2.into(), t.t.t.t.3.into(),
        	t.t.t.t.4.into(), t.t.t.t.5.into(), t.t.t.t.6.into(), t.t.t.t.7.into(),
        	t.t.t.t.8.into(), t.t.t.t.9.into(), t.t.t.t.10.into(), t.t.t.t.11.into(),
        	t.t.t.t.12.into(), t.t.t.t.13.into(), t.t.t.t.14.into(), t.t.t.t.15.into(),
        	t.t.t.t.16.into(), (),
        )
    }
}

impl From<BulletproofPreChallenge> for BulletproofPreChallengeV1 {
    fn from(t: BulletproofPreChallenge) -> Self {
        use mina_serialization_types::bulletproof_challenges::BulletproofPreChallenge as PC;
        match t {
            BulletproofPreChallenge::ScalarChallenge(v) => Versioned::new(PC::ScalarChallenge(v.into())),
            _ => unimplemented!(),
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
    ScalarChallengeVector2V1::new(
            (t.0.0.into(), t.0.1.into(), ())
        )
    }
}
impl From<ScalarChallengeVector2V1> for ScalarChallengeVector2 {
    fn from(t: ScalarChallengeVector2V1) -> Self {
        Self (
            (t.t.0.t.into(), t.t.1.t.into(), ())
        )
    }
}