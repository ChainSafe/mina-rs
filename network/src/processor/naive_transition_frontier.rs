// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! A naive implementation of the [TransitionFrontier]
//!

use super::*;
use mina_consensus::common::*;
use mina_merkle::*;
use mina_rs_base::{account::*, types::*, verifiable::Verifiable};
use proof_systems::{
    mina_hasher::{Fp, Hashable, ROInput},
    mina_signer::{self, NetworkId},
};
use tokio::sync::mpsc;

/// Merkle proof on mainnet
pub type MerkleProofMainnet = DefaultMerkleProof<
    AccountLegacy,
    Fp,
    MinaPoseidonMerkleHasherLegacy<AccountLegacy>,
    MinaPoseidonMerkleMergerLegacy,
>;

/// Dummy account type for berkeley net
#[derive(Debug, Clone)]
pub struct DummyAccount(Fp);

impl Hashable for DummyAccount {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

/// Dummy merkle hasher for berkeley net
pub struct DummyHasher;

impl MerkleHasher for DummyHasher {
    type Item = DummyAccount;
    type Hash = Fp;
    fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata) -> Self::Hash {
        item.0
    }
}

/// Merkle proof on berkeley net
/// TODO: Replace [DummyAccount], [DummyHasher] with [Account], [MinaPoseidonMerkleHasher<Account>]
/// respectively after we can deserialize [Account] from graphql API response
/// note that there are some new changes to the account hashing algorithm and the ledger hashes have
/// been changed as well. Those have to be fixed accordingly before switching to the real [Account]
pub type MerkleProofBerkeleyNet =
    DefaultMerkleProof<DummyAccount, Fp, DummyHasher, MinaPoseidonMerkleMerger>;

/// Struct that represents a naive implementation of the [TransitionFrontier]
#[derive(Debug, Clone, Default)]
pub struct NaiveTransitionFrontier<ProtocolState, Proof>
where
    ProtocolState: ProtocolStateHeader,
    Proof: MerkleProof,
{
    block_requester: Option<mpsc::Sender<QueryBlockRequest>>,
    best_chain: ProtocolStateChain<ProtocolState>,
    // TODO: Public APIs of the sparse merkle ledger is TBD,
    // replace Vec<Proof> with a new wrapper struct that exposes
    // proper public APIs
    sparse_merkle_ledger: Vec<Proof>,
}

impl<ProtocolState, Proof> NaiveTransitionFrontier<ProtocolState, Proof>
where
    ProtocolState: ProtocolStateHeader + Default,
    Proof: MerkleProof,
{
    /// Creates an instance
    pub fn new() -> Self {
        Self {
            block_requester: None,
            best_chain: Default::default(),
            sparse_merkle_ledger: vec![],
        }
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
impl TransitionFrontier for NaiveTransitionFrontier<ProtocolStateLegacy, MerkleProofMainnet> {
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
impl TransitionFrontier for NaiveTransitionFrontier<ProtocolState, MerkleProofBerkeleyNet> {
    type Block = (ProtocolState, Vec<MerkleProofBerkeleyNet>);

    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>) {
        self.block_requester = Some(sender);
    }

    async fn add_block(&mut self, block: Self::Block) -> anyhow::Result<()> {
        // TODO: Block verification
        let (block, proofs) = block;
        let state_hash_of_new_block = block.state_hash_fp();
        if self.best_chain.length() < 1 {
            self.best_chain.push(block)?;
            self.sparse_merkle_ledger = proofs;
        } else {
            let candidate_chains = vec![ProtocolStateChain(vec![block])];
            self.best_chain.select_secure_chain(candidate_chains)?;
            if self.best_chain.state_hash() == Some(state_hash_of_new_block) {
                self.sparse_merkle_ledger = proofs;
            }
        }
        Ok(())
    }
}
