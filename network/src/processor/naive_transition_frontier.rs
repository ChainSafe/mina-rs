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
    fp_from_radix_10,
    mina_hasher::{Fp, Hashable, ROInput},
    mina_signer::{self, CompressedPubKey, NetworkId},
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

/// protocol state + sparse merkle ledger
pub struct ProtocolStateWithSparseMerkleLedger(pub ProtocolState, pub Vec<MerkleProofBerkeleyNet>);

impl TryFrom<&serde_json::Value> for ProtocolStateWithSparseMerkleLedger {
    type Error = anyhow::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        use mina_rs_base::from_graphql_json::FromGraphQLJson;

        let protocol_state =
            ProtocolState::from_graphql_json(&json["bestChain"][0]["protocolState"])?;
        let mut sparse_merkle_ledger = vec![];

        if let Some(map) = json.as_object() {
            for (key, value) in map {
                if CompressedPubKey::from_address(key.as_str()).is_ok() {
                    let path: MerklePath = serde_json::from_value(value["merklePath"].clone())?;
                    sparse_merkle_ledger.push(path.to_proof(DummyAccount(fp_from_radix_10(
                        value["leafHash"].as_str().unwrap_or_default(),
                    )?))?);
                }
            }
        }

        Ok(Self(protocol_state, sparse_merkle_ledger))
    }
}

impl TryFrom<serde_json::Value> for ProtocolStateWithSparseMerkleLedger {
    type Error = anyhow::Error;

    fn try_from(json: serde_json::Value) -> Result<Self, Self::Error> {
        Self::try_from(&json)
    }
}

#[async_trait(?Send)]
impl TransitionFrontier for NaiveTransitionFrontier<ProtocolState, MerkleProofBerkeleyNet> {
    type Block = ProtocolStateWithSparseMerkleLedger;

    fn set_block_requester(&mut self, sender: mpsc::Sender<QueryBlockRequest>) {
        self.block_requester = Some(sender);
    }

    async fn add_block(&mut self, block: Self::Block) -> anyhow::Result<()> {
        // TODO: Block verification
        let ProtocolStateWithSparseMerkleLedger(block, proofs) = block;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_protocol_state_with_sparse_merkle_ledger() -> anyhow::Result<()> {
        const JSON_STR: &str = r###"
        {
            "bestChain": [
              {
                "protocolState": {
                  "previousStateHash": "3NK7nrZPxZ1vQRzNBnAfK5coYmdgygEHbRQ1GTT8f3o6b4dFWchL",
                  "consensusState": {
                    "blockCreator": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                    "blockHeight": "1",
                    "blockStakeWinner": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                    "epoch": "0",
                    "blockchainLength": "1",
                    "coinbaseReceiever": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                    "epochCount": "0",
                    "hasAncestorInSameCheckpointWindow": true,
                    "lastVrfOutput": "48H9Qk4D6RzS9kAJQX9HCDjiJ5qLiopxgxaS6xbDCWNaKQMQ9Y4C",
                    "minWindowDensity": "77",
                    "slot": "0",
                    "slotSinceGenesis": "0",
                    "superchargedCoinbase": true,
                    "totalCurrency": "1013238001000001000",
                    "nextEpochData": {
                      "epochLength": "2",
                      "lockCheckpoint": "3NK7nrZPxZ1vQRzNBnAfK5coYmdgygEHbRQ1GTT8f3o6b4dFWchL",
                      "seed": "2vafPBQ3zQdHUEDDnFGuiNvJz7s2MhTLJgSzQSnu5fnZavT27cms",
                      "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                      "ledger": {
                        "hash": "jwCg94ZjPhMA49CxbWySzs1uBPTGDwVn9gAa3XmiX43a9aJiV4T",
                        "totalCurrency": "1013238001000001000"
                      }
                    },
                    "stakingEpochData": {
                      "epochLength": "1",
                      "lockCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                      "seed": "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA",
                      "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                      "ledger": {
                        "hash": "jwCg94ZjPhMA49CxbWySzs1uBPTGDwVn9gAa3XmiX43a9aJiV4T",
                        "totalCurrency": "1013238001000001000"
                      }
                    }
                  },
                  "blockchainState": {
                    "bodyReference": "36bda176656cc3be96c3d317db7b4ac06fdbc7f4eedcd6efdd20e28143d67421",
                    "date": "1655755201000",
                    "snarkedLedgerHash": "jwCg94ZjPhMA49CxbWySzs1uBPTGDwVn9gAa3XmiX43a9aJiV4T",
                    "stagedLedgerAuxHash": "UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom",
                    "stagedLedgerHash": "jwCg94ZjPhMA49CxbWySzs1uBPTGDwVn9gAa3XmiX43a9aJiV4T",
                    "stagedLedgerPendingCoinbaseAux": "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
                    "stagedLedgerPendingCoinbaseHash": "2mzvU1MA5JjDUrLsKk6KFFj4CPn6bnHjN5Fi779U96JUbZNwoTvF",
                    "stagedLedgerProofEmitted": false,
                    "utcDate": "1655755201000",
                    "genesisLedgerHash": "jwCg94ZjPhMA49CxbWySzs1uBPTGDwVn9gAa3XmiX43a9aJiV4T"
                  }
                }
              }
            ],
            "genesisBlock": {
              "protocolState": {
                "blockchainState": {
                  "snarkedLedgerHash": "jwCg94ZjPhMA49CxbWySzs1uBPTGDwVn9gAa3XmiX43a9aJiV4T"
                }
              }
            },
            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg": {
              "merklePath": [
                {
                  "left": "18149634670481142922624879968372053792591888969047736011214449951083150195929",
                  "right": null
                },
                {
                  "left": "22812527483368966436451859546631477850058284910972714634031238147822271181086",
                  "right": null
                },
                {
                  "left": "18731053140433031831780515535822789442657502385916147531150966259032651557928",
                  "right": null
                },
                {
                  "left": "20296584525037576724025734247425163529078887029241403979115890832105851447954",
                  "right": null
                },
                {
                  "left": "14314268208843372012115947114633776645247936454149753766029212508710109739183",
                  "right": null
                },
                {
                  "left": "13933123071757088018369952977876920266582467979896977598876271267257596767130",
                  "right": null
                },
                {
                  "left": "23611346249079022662777343278399553287646033622284851532970302739997595309180",
                  "right": null
                },
                {
                  "left": "25655555643991292348665012320147587825788951391079344813930902649916449496026",
                  "right": null
                },
                {
                  "left": "23367493097668670616499692055560813744371830027635561536200164185069938692729",
                  "right": null
                },
                {
                  "left": "23615960292321574622151384655650231470598281536461180971667604451302160734488",
                  "right": null
                },
                {
                  "left": "17361130690372394222435410362568972462070643656170292626890542350515946346536",
                  "right": null
                },
                {
                  "left": "27389958092812561301776179073204036007430735018612680738137522809991440372965",
                  "right": null
                },
                {
                  "left": "23018631879597874403074159514054028005654277805202832202390416563222462874187",
                  "right": null
                },
                {
                  "left": "9618767215732590694420126776136765681230087687650241766635860959806667480610",
                  "right": null
                },
                {
                  "left": "26080540587126378565586696112871339589151127044608642948475517029694595225833",
                  "right": null
                },
                {
                  "left": "16763225678247801034948857325909312924298236123403522627065366245351091735698",
                  "right": null
                },
                {
                  "left": "4070610240312288384505292330058992176546392828821095025603146753462213582009",
                  "right": null
                },
                {
                  "left": "7844410138163611909844650973348731034544797929146144233184845993359232718206",
                  "right": null
                },
                {
                  "left": "12766165370795793087800119614599538600979734051558336879579021234592060212117",
                  "right": null
                },
                {
                  "left": "11896413817741888319571838357244390964227701441441970949674299652259492847171",
                  "right": null
                }
              ],
              "leafHash": "1089720686411837216975048380389083334047682594806546232185040873581918994577"
            },
            "B62qknCv9QdyAvt4Te58oo3nrZTacpEcjpJg1MV61r94h5rDPDUyPP8": {
              "merklePath": [
                {
                  "left": null,
                  "right": "20820645317443727902344485570929673159067134995813734812737542466704185166380"
                },
                {
                  "left": null,
                  "right": "6070058226423967848542667687195994870228580156076445851415585477490983567905"
                },
                {
                  "left": null,
                  "right": "13025881714863065780878019233344020717386613863001695032403326838140763451574"
                },
                {
                  "left": null,
                  "right": "26406644085334221415351197139742017178660304555729952121313684339031020073795"
                },
                {
                  "left": null,
                  "right": "22714114381845158081592050332343298087439318336175747792336659293182437439948"
                },
                {
                  "left": "886739365241573781743222380005772024947235923148429785648933641540507337285",
                  "right": null
                },
                {
                  "left": null,
                  "right": "1585407471992757123480439233777785008541493654396000974252319894037172939801"
                },
                {
                  "left": null,
                  "right": "22008176995696081806334005747377009921867969455686571374537011400467015539691"
                },
                {
                  "left": null,
                  "right": "19204583614551913515748772452473149061498174037958643654147150944549992192993"
                },
                {
                  "left": null,
                  "right": "25484130839727413544952259618836021988864276682580933670704376652578555557307"
                },
                {
                  "left": "17361130690372394222435410362568972462070643656170292626890542350515946346536",
                  "right": null
                },
                {
                  "left": "27389958092812561301776179073204036007430735018612680738137522809991440372965",
                  "right": null
                },
                {
                  "left": "23018631879597874403074159514054028005654277805202832202390416563222462874187",
                  "right": null
                },
                {
                  "left": "9618767215732590694420126776136765681230087687650241766635860959806667480610",
                  "right": null
                },
                {
                  "left": "26080540587126378565586696112871339589151127044608642948475517029694595225833",
                  "right": null
                },
                {
                  "left": "16763225678247801034948857325909312924298236123403522627065366245351091735698",
                  "right": null
                },
                {
                  "left": "4070610240312288384505292330058992176546392828821095025603146753462213582009",
                  "right": null
                },
                {
                  "left": "7844410138163611909844650973348731034544797929146144233184845993359232718206",
                  "right": null
                },
                {
                  "left": "12766165370795793087800119614599538600979734051558336879579021234592060212117",
                  "right": null
                },
                {
                  "left": "11896413817741888319571838357244390964227701441441970949674299652259492847171",
                  "right": null
                }
              ],
              "leafHash": "16769595356273175443822303155900591811938380130045021592789187496698437646609"
            }
          }
        "###;
        let json: serde_json::Value = serde_json::from_str(JSON_STR)?;
        let ProtocolStateWithSparseMerkleLedger(_, sparse_merkle_ledger) =
            ProtocolStateWithSparseMerkleLedger::try_from(&json)?;
        anyhow::ensure!(sparse_merkle_ledger.len() == 2);
        Ok(())
    }
}
