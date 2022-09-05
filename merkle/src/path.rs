// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! This module contains traits, structs and utilities of merkle path

use crate::*;
use proof_systems::{mina_hasher::Fp, FpJson};
use serde::{Deserialize, Serialize};

/// Node struct on [MerklePath]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerklePathNode {
    /// left child
    pub left: Option<FpJson>,
    /// right child
    pub right: Option<FpJson>,
}

impl MerklePathNode {
    /// Check if node has left child, throw err if node is invalid
    pub fn has_left_child(&self) -> anyhow::Result<bool> {
        if self.left.is_some() {
            if self.right.is_none() {
                Ok(true)
            } else {
                Err(anyhow::Error::msg("Invalid node, 2 hashes found"))
            }
        } else if self.right.is_some() {
            Ok(false)
        } else {
            Err(anyhow::Error::msg("Invalid node, 0 hashes found"))
        }
    }
}

/// Merkle path that can be serded from/to mina graphql api response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerklePath(pub Vec<MerklePathNode>);

impl MerklePath {
    /// Convert merkle path to merkle proof data structure
    pub fn to_proof<Item, Hasher, Merger>(
        &self,
        item: Item,
    ) -> anyhow::Result<DefaultMerkleProof<Item, Fp, Hasher, Merger>>
    where
        Hasher: MerkleHasher<Item = Item, Hash = Fp>,
        Merger: MerkleMerger<Hash = Fp>,
    {
        let mut peer_indices = Vec::with_capacity(self.0.len());
        let mut peer_hashes = Vec::with_capacity(self.0.len());
        let mut row_index = 0;
        let mut peer_index = 0;
        let mut node_is_left = false;
        for node in self.0.iter().rev() {
            row_index *= 2;
            node_is_left = node.has_left_child()?;
            let mut peer_row_index = row_index;
            if node_is_left {
                peer_row_index += 1;
            } else {
                row_index += 1;
            }
            let height_above = peer_indices.len();
            peer_index = 2_usize.pow(height_above as u32 + 1) - 1 + peer_row_index;
            peer_indices.insert(0, peer_index);
            peer_hashes.insert(
                0,
                if node_is_left {
                    node.left.clone().map(|i| i.0)
                } else {
                    node.right.clone().map(|i| i.0)
                },
            );
        }
        let item_index = if node_is_left {
            peer_index - 1
        } else {
            peer_index + 1
        };
        Ok(DefaultMerkleProof::new(
            item_index,
            item,
            peer_indices,
            peer_hashes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::BigInteger256;
    use proof_systems::mina_hasher::{Hashable, ROInput};

    #[derive(Debug, Clone)]
    struct TestLeafNode(Fp);

    impl Hashable for TestLeafNode {
        type D = ();

        fn to_roinput(&self) -> mina_hasher::ROInput {
            ROInput::new().append_field(self.0)
        }

        fn domain_string(_: Self::D) -> Option<String> {
            None
        }
    }

    struct TestHasher;

    impl MerkleHasher for TestHasher {
        type Item = TestLeafNode;
        type Hash = Fp;
        fn hash(item: &Self::Item, _: MerkleTreeNodeMetadata) -> Self::Hash {
            item.0
        }
    }

    #[test]
    fn merkle_path_serde_round_trip_0() -> anyhow::Result<()> {
        // query MyQuery {
        //   account(publicKey: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg") {
        //     leafHash
        //     merklePath {
        //       left
        //       right
        //     }
        //   }
        // }
        merkle_path_serde_round_trip_inner(
            r###"
            [
          {
            "left": "15955265048676495861308121109313805689802008176208259245703497109342504292532",
            "right": null
          },
          {
            "left": "6498763974107592226078643188118450325531711052055473050928238305750518827436",
            "right": null
          },
          {
            "left": "14798323239743086727237666336013123038721545962424159269965881035430827192139",
            "right": null
          },
          {
            "left": "7281277558774986200833727548377711319104176574193698011055354172757414578012",
            "right": null
          },
          {
            "left": "21421284228608452991592400544447852683183702268529403235483845926047915727702",
            "right": null
          },
          {
            "left": "2373083531277560524519693240556516270149478088495043390041045484316846594085",
            "right": null
          },
          {
            "left": "26974825521826223811673310242661188830681317765743459401786571534371307740265",
            "right": null
          },
          {
            "left": "23028533109832873726991259774592221117314384147065571789280148718227955970625",
            "right": null
          },
          {
            "left": "21292364479605775942941570127304682174959790505058210586394968767417146753887",
            "right": null
          },
          {
            "left": "28472663554086566858238873097370421776162561424155685293511315926906146138683",
            "right": null
          },
          {
            "left": "20113322215021414848485614258568830974362579973762267037153302942792566260677",
            "right": null
          },
          {
            "left": "18802032621905225316071440374266472002222303392354416281474392930306826877176",
            "right": null
          },
          {
            "left": "1467407896713240134527372886208550131963978332314558689172127325748899529534",
            "right": null
          },
          {
            "left": "4760908914501509089647845483160027108221626446599138668534607880831037931734",
            "right": null
          },
          {
            "left": "3559864511735229366736965808627021136452902027018040714387870065555750662592",
            "right": null
          },
          {
            "left": "615176793736335606813478083405278295088475976207230851624897585321667196394",
            "right": null
          },
          {
            "left": "15148755790521665409450720973240562522589192313825539361763883304907039908036",
            "right": null
          },
          {
            "left": "182539158631822393410010600702481573763869609447443995876874694723165731778",
            "right": null
          },
          {
            "left": "25680018261350126780251378752072955428530054073224897810410636836484248676771",
            "right": null
          },
          {
            "left": "18920144742871483864781265699787600756474192693599909882198533915048113470039",
            "right": null
          }
        ]
            "###,
            fp_from_radix_10(
                "6108279148345709945760643065760116305669855696483393142482838525482893407897",
            )?,
        )
    }

    #[test]
    fn merkle_path_serde_round_trip_991() -> anyhow::Result<()> {
        // query MyQuery {
        //   account(publicKey: "B62qknCv9QdyAvt4Te58oo3nrZTacpEcjpJg1MV61r94h5rDPDUyPP8") {
        //     leafHash
        //     merklePath {
        //       left
        //       right
        //     }
        //   }
        // }
        merkle_path_serde_round_trip_inner(
            r###"
            [
        {
          "left": null,
          "right": "15254873505849811741689626954389304158553805646690506999421186472202908262069"
        },
        {
          "left": null,
          "right": "6868208116234910456158077727635043574608503751468137393078539689167155855062"
        },
        {
          "left": null,
          "right": "3659366143391409547667934228041227082981384023489323900638421291174295405100"
        },
        {
          "left": null,
          "right": "12104132418349166569006286441269554412362827050494108853521921756835177071275"
        },
        {
          "left": null,
          "right": "28480770716386236230212207530599536716043465735372737587320010232108223797980"
        },
        {
          "left": "20248922383533254090683247006130124522275245120984550982620962056023423157459",
          "right": null
        },
        {
          "left": null,
          "right": "13413970865917873626333806246818812730351919615943449156726722855671475482890"
        },
        {
          "left": null,
          "right": "28283383752365502743850604975904119903467085600897467741380913860160676397870"
        },
        {
          "left": null,
          "right": "16690269690915293215023485432324548878559331189583937447025110242021791208623"
        },
        {
          "left": null,
          "right": "9525094856163906997712911171166325034800790806579383452855016457000779480969"
        },
        {
          "left": "20113322215021414848485614258568830974362579973762267037153302942792566260677",
          "right": null
        },
        {
          "left": "18802032621905225316071440374266472002222303392354416281474392930306826877176",
          "right": null
        },
        {
          "left": "1467407896713240134527372886208550131963978332314558689172127325748899529534",
          "right": null
        },
        {
          "left": "4760908914501509089647845483160027108221626446599138668534607880831037931734",
          "right": null
        },
        {
          "left": "3559864511735229366736965808627021136452902027018040714387870065555750662592",
          "right": null
        },
        {
          "left": "615176793736335606813478083405278295088475976207230851624897585321667196394",
          "right": null
        },
        {
          "left": "15148755790521665409450720973240562522589192313825539361763883304907039908036",
          "right": null
        },
        {
          "left": "182539158631822393410010600702481573763869609447443995876874694723165731778",
          "right": null
        },
        {
          "left": "25680018261350126780251378752072955428530054073224897810410636836484248676771",
          "right": null
        },
        {
          "left": "18920144742871483864781265699787600756474192693599909882198533915048113470039",
          "right": null
        }
      ]
            "###,
            fp_from_radix_10(
                "23200403512741972025856044490391904381197742159081194820723357079179017951910",
            )?,
        )
    }

    fn merkle_path_serde_round_trip_inner(json_str: &str, leaf_hash: Fp) -> anyhow::Result<()> {
        let json_value: serde_json::Value = serde_json::from_str(json_str)?;
        let obj: MerklePath = serde_json::from_value(json_value.clone())?;
        assert_eq!(serde_json::to_value(&obj)?, json_value);

        let proof: DefaultMerkleProof<TestLeafNode, Fp, TestHasher, MinaPoseidonMerkleMerger> =
            obj.to_proof(TestLeafNode(leaf_hash))?;

        // This is the merkle root of the genesis ledger
        assert!(proof.verify(&fp_from_radix_10(
            "20038089104619582172254839672519820202817728273163650761198500757943363448868"
        )?));
        Ok(())
    }

    fn fp_from_radix_10(s: &str) -> anyhow::Result<Fp> {
        use std::str::FromStr;

        let big = num::BigUint::from_str(s)?;
        let big256: BigInteger256 = big.try_into().unwrap();
        Ok(big256.into())
    }
}
