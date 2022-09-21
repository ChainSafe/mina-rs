// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! A naive implementation of the [TransitionFrontier]
//!

use super::*;
use mina_consensus::common::*;
use mina_merkle::{
    DefaultMerkleProof, MerkleProof, MinaPoseidonMerkleHasher, MinaPoseidonMerkleHasherLegacy,
    MinaPoseidonMerkleMerger, MinaPoseidonMerkleMergerLegacy,
};
use mina_rs_base::{account::*, types::*, verifiable::Verifiable};
use proof_systems::{
    mina_hasher::Fp,
    mina_signer::{self, NetworkId},
};
use tokio::sync::mpsc;

/// Struct that represents a naive implementation of the [TransitionFrontier]
#[derive(Debug, Clone, Default)]
pub struct NaiveTransitionFrontier<ProtocolState, Proof>
where
    ProtocolState: ProtocolStateHeader,
    Proof: MerkleProof,
{
    block_requester: Option<mpsc::Sender<QueryBlockRequest>>,
    best_chain: ProtocolStateChain<ProtocolState>,
    sparse_merkle_ledger: Vec<Proof>,
}

impl<ProtocolState, Proof> NaiveTransitionFrontier<ProtocolState, Proof>
where
    ProtocolState: ProtocolStateHeader + Default,
    Proof: MerkleProof + Default,
{
    /// Creates an instance
    pub fn new() -> Self {
        Default::default()
    }

    /// Gets the current best chain being selected
    pub fn get_best_chain(&self) -> &ProtocolStateChain<ProtocolState> {
        &self.best_chain
    }

    /// Gets the sparse merkle ledger which is essentially a collection of merkle proofs
    pub fn get_sparse_merkle_ledger(&self) -> &[Proof] {
        self.sparse_merkle_ledger.as_slice()
    }
}

#[async_trait(?Send)]
impl TransitionFrontier
    for NaiveTransitionFrontier<
        ProtocolStateLegacy,
        DefaultMerkleProof<
            AccountLegacy,
            Fp,
            MinaPoseidonMerkleHasherLegacy<AccountLegacy>,
            MinaPoseidonMerkleMergerLegacy,
        >,
    >
{
    type Block = ExternalTransition;

    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>) {
        self.block_requester = Some(sender);
    }

    async fn add_block(&mut self, block: Self::Block) -> anyhow::Result<()> {
        let mut ctx = mina_signer::create_legacy::<SignedCommandPayload>(NetworkId::MAINNET);
        anyhow::ensure!(block.verify(&mut ctx), "block verification failure");
        if self.best_chain.length() < 1 {
            self.best_chain.push(block.protocol_state)?;
        } else {
            let candidate_chains = vec![ProtocolStateChain(vec![block.protocol_state])];
            self.best_chain.select_secure_chain(candidate_chains)?;
        }

        // FIXME: We're not able to fetch merkle proofs from mainnet graphql API
        Ok(())
    }
}

#[async_trait(?Send)]
impl TransitionFrontier
    for NaiveTransitionFrontier<
        ProtocolState,
        DefaultMerkleProof<
            Account,
            Fp,
            MinaPoseidonMerkleHasher<Account>,
            MinaPoseidonMerkleMerger,
        >,
    >
{
    type Block = ProtocolState;

    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>) {
        self.block_requester = Some(sender);
    }

    async fn add_block(&mut self, block: Self::Block) -> anyhow::Result<()> {
        // TODO: Block verification
        if self.best_chain.length() < 1 {
            self.best_chain.push(block)?;
        } else {
            let candidate_chains = vec![ProtocolStateChain(vec![block])];
            self.best_chain.select_secure_chain(candidate_chains)?;
        }
        Ok(())
    }
}
