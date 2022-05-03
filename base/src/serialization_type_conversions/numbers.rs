// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::numbers::*;
use mina_serialization_types::v1::{
    AccountNonceV1, AmountV1, BlockTimeV1, CharV1, DeltaV1, ExtendedU32 as ExtendedU32V1,
    ExtendedU64_3, GlobalSlotNumberV1, Hex64V1, LengthV1, TokenIdV1,
};
use versioned::*;

impl_from_for_newtype!(Amount, AmountV1);
impl_from_for_newtype!(Amount, ExtendedU64_3);

impl_from_for_newtype!(Length, LengthV1);

impl_from_for_newtype!(Delta, DeltaV1);

impl_from_for_newtype!(ExtendedU32, ExtendedU32V1);

impl_from_for_newtype!(TokenId, TokenIdV1);

impl_from_for_newtype!(AccountNonce, AccountNonceV1);
impl From<AccountNonce> for ExtendedU32V1 {
    fn from(t: AccountNonce) -> Self {
        (t.0 as i32).into()
    }
}
impl From<ExtendedU32V1> for AccountNonce {
    fn from(t: ExtendedU32V1) -> Self {
        Self(t.t.t as u32)
    }
}

impl_from_for_newtype!(BlockTime, BlockTimeV1);

impl_from_for_newtype!(GlobalSlotNumber, GlobalSlotNumberV1);
impl From<GlobalSlotNumber> for ExtendedU32V1 {
    fn from(t: GlobalSlotNumber) -> Self {
        (t.0 as i32).into()
    }
}
impl From<ExtendedU32V1> for GlobalSlotNumber {
    fn from(t: ExtendedU32V1) -> Self {
        Self(t.t.t as u32)
    }
}

impl_from_for_newtype!(Hex64, Hex64V1);

impl_from_for_newtype!(Char, CharV1);
