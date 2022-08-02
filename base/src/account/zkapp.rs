// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! zk app

use ark_ff::{BigInteger256, One, Zero};
use derive_more::{From, Into};
use once_cell::sync::OnceCell;
use proof_systems::{
    mina_hasher::{self, Fp, Hashable, Hasher, ROInput},
    ChunkedROInput, ToChunkedROInput,
};

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

impl ToChunkedROInput for ZkApp {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
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
        ChunkedROInput::new()
            .append_field(FP1.into())
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
            .append_packed(Fp::zero(), 1)
            .append_packed(Fp::zero(), 32)
            .append_packed(Fp::zero(), 32)
    }
}

/// Wrapper of [Option<ZkApp>] that implements [Hashable]
#[derive(Debug, Clone)]
pub struct ZkAppOptionHashableWrapper<'a>(pub &'a Option<ZkApp>);

impl<'a> Hashable for ZkAppOptionHashableWrapper<'a> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        if let Some(v) = self.0 {
            v
        } else {
            ZkApp::borrow_default()
        }
        .roinput()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaZkappAccount".into())
    }
}

impl<'a> ToChunkedROInput for ZkAppOptionHashableWrapper<'a> {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new().append_field({
            let mut hasher = mina_hasher::create_kimchi(());
            hasher.hash(self)
        })
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

impl ToChunkedROInput for ZkAppUri {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        // FIXME: This is only for default hash
        ChunkedROInput::new().append_packed(Fp::one(), 1)
    }
}

/// Wrapper of [Option<ZkAppUri>] that implements [Hashable]
#[derive(Debug, Clone)]
pub struct ZkAppUriOptionHashableWrapper<'a>(pub &'a Option<ZkAppUri>);

impl<'a> Hashable for ZkAppUriOptionHashableWrapper<'a> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        if let Some(v) = self.0 {
            v
        } else {
            ZkAppUri::borrow_default()
        }
        .roinput()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("MinaZkappUri".into())
    }
}

impl<'a> ToChunkedROInput for ZkAppUriOptionHashableWrapper<'a> {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new().append_field({
            let mut hasher = mina_hasher::create_kimchi(());
            hasher.hash(&ZkAppUriOptionHashableWrapper(self.0))
        })
    }
}
