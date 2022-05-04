// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::numbers::*;
use mina_serialization_types::v1::{
    AccountNonceV1, AmountV1, BlockTimeV1, DeltaV1,
    GlobalSlotNumberV1, Hex64V1, LengthV1, TokenIdV1,
};
use versioned::Versioned;

impl From<Amount> for AmountV1 {
    fn from(t: Amount) -> Self {
        AmountV1::new(Versioned::new(t.0))
    }
}

impl From<Length> for LengthV1 {
    fn from(t: Length) -> Self {
        LengthV1::new(Versioned::new(t.0))
    }
}

impl From<Delta> for DeltaV1 {
    fn from(t: Delta) -> Self {
        DeltaV1::new(Versioned::new(t.0))
    }
}

impl From<TokenId> for TokenIdV1 {
    fn from(t: TokenId) -> Self {
        TokenIdV1::new(Versioned::new(Versioned::new(t.0)))
    }
}

impl From<AccountNonce> for AccountNonceV1 {
    fn from(t: AccountNonce) -> Self {
        AccountNonceV1::new(Versioned::new(t.0))
    }
}

impl From<BlockTime> for BlockTimeV1 {
    fn from(t: BlockTime) -> Self {
        BlockTimeV1::new(Versioned::new(t.0))
    }
}

impl From<GlobalSlotNumber> for GlobalSlotNumberV1 {
    fn from(t: GlobalSlotNumber) -> Self {
        GlobalSlotNumberV1::new(Versioned::new(t.0))
    }
}

impl From<Hex64> for Hex64V1 {
    fn from(t: Hex64) -> Self {
        Hex64V1::new(t.0)
    }
}
