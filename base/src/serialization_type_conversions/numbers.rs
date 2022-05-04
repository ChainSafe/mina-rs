// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::numbers::*;
use mina_serialization_types::v1::{
    AccountNonceV1, AmountV1, BlockTimeV1, DeltaV1,
    GlobalSlotNumberV1, Hex64V1, LengthV1, TokenIdV1,
};


use versioned::impl_from_for_newtype;

impl_from_for_newtype!(Amount, AmountV1);

impl_from_for_newtype!(Length, LengthV1);

impl_from_for_newtype!(Delta, DeltaV1);

impl_from_for_newtype!(TokenId, TokenIdV1);

impl_from_for_newtype!(AccountNonce, AccountNonceV1);

impl_from_for_newtype!(BlockTime, BlockTimeV1);

impl_from_for_newtype!(GlobalSlotNumber, GlobalSlotNumberV1);

impl_from_for_newtype!(Hex64, Hex64V1);
