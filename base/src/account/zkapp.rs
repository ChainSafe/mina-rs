// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! zk app

use ark_ff::{BigInteger256, Zero};
use derive_more::{From, Into};
use once_cell::sync::OnceCell;
use proof_systems::mina_hasher::{Fp, Hashable, ROInput};

/// FIXME: Doc
#[derive(Default, Debug, Clone, From, Into)]
pub struct ZkApp(());

impl<'a> ZkApp {
    /// Get a borrow of the default value
    pub fn borrow_default() -> &'a Self {
        static INSTANCE: OnceCell<ZkApp> = OnceCell::new();
        INSTANCE.get_or_init(Self::default)
    }
}

impl Hashable for ZkApp {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        // FIXME: This is only for default hash
        // 19777675955122618431670853529822242067051263606115426372178827525373304476695
        const FP1: BigInteger256 = BigInteger256::new([
            4943954750823961623,
            15869131315174455918,
            2268342156128405549,
            3150765558510464065,
        ]);
        // 15836741414052211301983886193856353162526040956490609761139212467629447291325
        const FP2: BigInteger256 = BigInteger256::new([
            17064760232198382013,
            7053532856486746066,
            14503003448090529760,
            2522938464542289201,
        ]);
        roi.append_field(FP1.into())
            .append_field(FP1.into())
            .append_field(FP1.into())
            .append_field(FP1.into())
            .append_field(FP1.into())
            .append_field(FP2.into())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero())
            .append_field(Fp::zero());

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaZkappAccount".into())
    }
}

/// Wrapper of [Option<ZkApp>] that implements [Hashable]
#[derive(Debug, Clone)]
pub struct ZkAppOptionHashableWrapper<'a>(pub &'a Option<ZkApp>);

impl<'a> Hashable for ZkAppOptionHashableWrapper<'a> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(if let Some(v) = self.0 {
            v
        } else {
            ZkApp::borrow_default()
        });
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

/// FIXME: Doc
#[derive(Default, Debug, Clone, From, Into)]
pub struct ZkAppUri(());

impl<'a> ZkAppUri {
    /// Get a borrow of the default value
    pub fn borrow_default() -> &'a Self {
        static INSTANCE: OnceCell<ZkAppUri> = OnceCell::new();
        INSTANCE.get_or_init(Self::default)
    }
}

impl Hashable for ZkAppUri {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        // FIXME: This is only for default hash
        roi.append_bool(true);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("MinaZkappUri".into())
    }
}

/// Wrapper of [Option<ZkAppUri>] that implements [Hashable]
#[derive(Debug, Clone)]
pub struct ZkAppUriOptionHashableWrapper<'a>(pub &'a Option<ZkAppUri>);

impl<'a> Hashable for ZkAppUriOptionHashableWrapper<'a> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(if let Some(v) = self.0 {
            v
        } else {
            ZkAppUri::borrow_default()
        });
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
