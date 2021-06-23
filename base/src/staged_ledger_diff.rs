// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// See mina/src/lib/staged_ledger_diff/staged_ledger_diff.ml for original impl

use serde::{Serialize, Deserialize};
use serde_versions_derive::version;

use crate::transaction_status::{WithStatus, InternalCommandBalanceData};

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct StagedLedgerDiff{
	diff: Diff,
}


//////////////////////////////////////////


#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct AtMostOne<T: Clone>(AtMostOneInner<T>);

#[derive(Clone, Serialize, Deserialize)]
enum AtMostOneInner<T: Clone> {
	Zero,
	One(Option<T>),
}



#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct AtMostTwo<T: Clone>(AtMostTwoInner<T>);

#[derive(Clone, Serialize, Deserialize)]
enum AtMostTwoInner<T: Clone> {
	Zero,
	One(Option<T>),
	Two(Option<(T, Option<T>)>)
}


//////////////////////////////////////////

/// Fee transfer wrapped and reversioned
#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct Ft(crate::coinbase_fee_transfer::CoinbaseFeeTransfer);

//////////////////////////////////////////


#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct PreDiffTwo<A: Clone, B: Clone> {
	completed_works: Vec<A>,
	commands: Vec<B>,
	coinbase: AtMostTwo<Ft>,
	internal_command_balances: Vec<InternalCommandBalanceData>
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct PreDiffOne<A: Clone, B: Clone> {
	completed_works: Vec<A>,
	commands: Vec<B>,
	coinbase: AtMostOne<Ft>,
	internal_command_balances: Vec<InternalCommandBalanceData>
}

//////////////////////////////////////////

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct PreDiffWithAtMostTwoCoinbase(PreDiffTwo<TransactionSnarkWork, WithStatus<UserCommand>>);


#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct PreDiffWithAtMostOneCoinbase(PreDiffOne<TransactionSnarkWork, WithStatus<UserCommand>>);

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Diff(PreDiffWithAtMostTwoCoinbase, Option<PreDiffWithAtMostOneCoinbase>);
