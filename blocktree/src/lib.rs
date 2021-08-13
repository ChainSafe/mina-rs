// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

mod error;
mod node;
mod leafmap;

use error::ForkTreeError;
use leafmap::LeafMap;
use node::Node;
use std::iter::FromIterator;

use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap, DashSet,
};
use mina_crypto::hash::{Hashable, StateHash};
use mina_rs_base::protocol_state::ProtocolState;
use std::time::{SystemTime, UNIX_EPOCH};

type NodeRef<'a> = Option<Ref<'a, StateHash, Node>>;
type NodeMutRef<'a> = Option<RefMut<'a, StateHash, Node>>;

// TODO: implement this once we have the use case finalized.
pub trait BlockTree {
    // TODO: can probably just store the hash and block number
    /// adds a block to the blocktree. should error if the parent node isn't found in the blocktree
    fn add_block(&mut self, block: ProtocolState) -> Result<(), ForkTreeError>;
    /// return the list of `ProtocolState`s in between (and including) the start and end `StateHash`es. error if `start` or `end` aren't in the blocktree or `end`  isn't a descendant of `start`
    fn sub_chain(start: StateHash, end: StateHash) -> Result<Vec<ProtocolState>, ForkTreeError>;
    /// this is essentially the `selectLongerChain` algorithm in spec 4.2, it returns the highest block in the tree, if there are multiple highest blocks it uses the tiebreaker logic specified
    fn get_highest_block() -> Option<ProtocolState>;
    /// this returns the highest common ancestor between two nodes (ie, if b1 and b2 are on two different forks, then return the block where they forked)
    /// this can be used as input into `subchain` to get chains which can be used as input for `is_short_range` and `get_min_density`
    fn highest_common_ancestor<'a>(
        b1: &'a ProtocolState,
        b2: &'a ProtocolState,
    ) -> &'a ProtocolState;
    /// this is probably not needed at the moment, but eventually we will need to prune the tree once a block has a very high probability of not being reverted. We can also prune a long-range fork if we deem it invalid
    fn prune();
}

// TODO: replace this with actual DB handle
#[derive(Clone)]
struct Database;

// ForkTree maintains a list of candidate chains
// received from peers and represents the current state
// with all possible blocks.
// Design: ForkTree along with its root node and the subsequent
// child node only maintains unique state hashes of the nodes
// The actual node themselves are stored in a concurrent hashmap.
#[derive(Clone)]
struct ForkTree {
    nodemap: DashMap<StateHash, Node>,
    // the root node, rooted at the most succint state.
    root: StateHash,
    // collection of blocks which don't have any children
    leaves: LeafMap,
    db: Database,
    // only store the StatHash and perform lookup
    node_cache: DashSet<StateHash>,
}

fn current_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

impl ForkTree {
    // from_root initializes a blocktree with a root block.
    // The root block is always the most recently finalized block
    // i.e., the genesis block if the node is just strating.
    // in case of mina it's the first block in the last 290th block
    // TODO confirm this?
    fn from_root(root_hash: &ProtocolState, db: Database) -> Self {
        let root_hash = root_hash.hash();
        let root = Node {
            hash: root_hash,
            parent: None,
            children: vec![],
            depth: 0,
            arrival_time: current_time(),
        };

        let nodemap = DashMap::new();

        nodemap.insert(root_hash, root);

        ForkTree {
            nodemap,
            root: root_hash,
            leaves: LeafMap::new(),
            db: Database,
            node_cache: DashSet::new(),
        }
    }

    // GenesisHash returns the hash of the genesis block
    // FIXME: self.root is incorrect as the chain root is
    // updated to a more recent block and self.root is reassigned.
    // This might need to read the db to retrieve the genesis state hash.
    fn genesis_hash(&self) -> StateHash {
        self.root
    }

    // add_block should do the following:
    // 1. gets parent
    // 2. creates a node
    // 3. inserts the node into nodemap (DashMap)
    // 4. adds the node into parent's children
    // 5. removes from leaves field, the parent (which now has children)
    //    and inserts the new node which does not have children.
    fn add_block(&mut self, header: ProtocolState, arrival_time: u64) -> Result<(), ForkTreeError> {
        // check if it already exists
        if self.get_node(header.hash()).is_some() {
            return Err(ForkTreeError::BlockExists);
        }

        let (parent_hash, parent_depth) = {
            let parent = self.nodemap.get_mut(&header.previous_state_hash);
            if parent.is_none() {
                return Err(ForkTreeError::ParentNotFound);
            }

            let parent_hash = parent.as_deref().unwrap().hash;
            let parent_depth = parent.as_deref().unwrap().depth;

            (parent_hash, parent_depth)
        };

        let depth = parent_depth + 1;

        let node = Node {
            hash: header.hash(),
            parent: Some(parent_hash),
            children: vec![],
            depth,
            arrival_time,
        };

        let node_hash = node.hash.clone();

        self.nodemap.insert(node_hash, node);
        let mut parent = self.nodemap.get_mut(&header.previous_state_hash);
        parent.as_deref_mut().unwrap().add_child(node_hash);

        self.leaves.replace(&parent.unwrap().hash, node_hash);

        Ok(())
    }

    // Rewind rewinds the block tree by the given height. If the blocktree is less than the given height,
    // it will only rewind until the blocktree has one node.
    fn rewind(&mut self, num_blocks: usize) {
        for i in 0..num_blocks {
            let deepest_leaf = self.leaves.deepest_leaf(&self.nodemap).unwrap();
            let deepest_depth = self.get_node(deepest_leaf).unwrap().depth;
            for leaf_hash in self.leaves.nodes(&self.nodemap) {
                let leaf = {
                    let leaf = self.get_node(leaf_hash).unwrap();
                    if leaf.parent.is_none() || leaf.depth < deepest_depth {
                        continue;
                    }
                    leaf.clone()
                };

                self.leaves.replace(&leaf_hash, leaf.parent.unwrap());
                let mut leaf_parent = self.get_node_mut(leaf.parent.unwrap()).unwrap();
                leaf_parent.delete_child(leaf_hash)
            }
        }
    }

    // Returns all block's hashes with the depth of the given hash plus one
    // To find all blocks at a depth matching a certain block, pass in that block's
    // parent hash
    fn get_all_blocks_at_depth(&self, hash: StateHash) -> Vec<StateHash> {
        let mut hashes = vec![];

        if self.get_node(hash).is_none() {
            return hashes;
        }

        let depth = 1 + self.get_node(hash).unwrap().depth;

        let root_node = self.nodemap.get(&self.root).unwrap();

        if root_node.depth == depth {
            hashes.push(self.root);
            return hashes;
        }

        let root_node = self.nodemap.get(&self.root).unwrap();

        root_node.get_nodes_with_depth(depth, &self.nodemap)
    }

    fn set_in_cache(&mut self, block: StateHash) {
        if !self.node_cache.contains(&block) {
            self.node_cache.insert(block);
        }
    }

    fn deepest_leaf(&self) -> Option<StateHash> {
        self.leaves.deepest_leaf(&self.nodemap)
    }

    // getNode finds and returns a node based on its Hash. Returns `None` if not found
    fn get_node(&self, hash: StateHash) -> NodeRef<'_> {
        self.nodemap.get(&hash)
    }

    // get_node_mut finds and returns a node based on its Hash. Returns None if not found
    fn get_node_mut(&mut self, hash: StateHash) -> NodeMutRef<'_> {
        if self.root == hash {
            let node = self.nodemap.get_mut(&self.root);
            return node;
        }

        for leaf in self.leaves.nodes(&self.nodemap) {
            if leaf == hash {
                let node = self.nodemap.get_mut(&self.root);
                return node;
            }
        }

        if let Some(root_node) = self.nodemap.get_mut(&self.root) {
            for c in &root_node.children {
                let child_node = self.nodemap.get_mut(&c);
                if let Some(n) = child_node {
                    return Some(n);
                }
            }
        }

        None
    }

    /// Sets the given hash as the new blocktree root, removing all nodes
    /// that are not the new root node or its descendant.
    fn prune(&mut self, finalized: StateHash) -> Vec<StateHash> {
        let pruned = vec![];
        if finalized == self.root {
            return pruned;
        }

        let n = if let Some(mut n) = self.nodemap.get(&finalized) {
            n
        } else {
            return pruned;
        };

        let root_node = self.nodemap.get(&self.root).unwrap();
        let pruned = root_node.prune(&n, &self.nodemap);
        self.root = n.hash;
        let leaves = n.get_leaves(&self.nodemap);

        self.leaves.map = DashSet::from_iter(leaves);

        return pruned;
    }

    fn longest_path(&self) -> Vec<StateHash> {
        let mut cur_leaf = self.leaves.deepest_leaf(&self.nodemap);
        let mut path = vec![];

        loop {
            cur_leaf = if let Some(cur_leaf) = cur_leaf {
                path.push(cur_leaf.clone());
                let cur_leaf_node = self.nodemap.get(&cur_leaf).unwrap(); // crate::get_node(&cur_leaf).unwrap();
                cur_leaf_node.parent
            } else {
                path.reverse();
                return path;
            };
        }
    }

    // sub_chain returns the path from the node with Hash start to the node with Hash end
    fn sub_chain(&self, start: StateHash, end: StateHash) -> Result<Vec<StateHash>, ForkTreeError> {
        let sn = self.get_node(start);
        if sn.is_none() {
            return Err(ForkTreeError::StartNodeNotFound);
        }

        let en = self.get_node(end);
        if en.is_none() {
            return Err(ForkTreeError::EndNodeNotFound);
        }

        return sn.unwrap().sub_chain(en.as_deref(), &self.nodemap);
    }

    fn sub_block_chain(
        &self,
        start: StateHash,
        end: StateHash,
    ) -> Result<Vec<StateHash>, ForkTreeError> {
        let sc = self.sub_chain(start, end)?;

        let mut bc = vec![];
        for node_hash in sc {
            bc.push(node_hash);
        }

        Ok(bc)
    }

    // Returns the hash of the deepest block in the blocktree
    // If there is multiple deepest blocks, it returns the one with the earliest arrival time.
    fn deepest_block_hash(&self) -> Option<StateHash> {
        if self.leaves.map.is_empty() {
            // TODO: don't know why the go impl returns
            // an emoty Hash object. isn't that incorrect?
            return None;
        }

        let deepest = self.leaves.deepest_leaf(&self.nodemap);
        return deepest;
    }

    fn is_descendant_of(&self, parent: StateHash, child: StateHash) -> Result<bool, ForkTreeError> {
        let pn = self.get_node(parent);
        if pn.is_none() {
            return Err(ForkTreeError::StartNodeNotFound);
        }
        let cn = self.get_node(child);
        if cn.is_none() {
            return Err(ForkTreeError::EndNodeNotFound);
        }

        Ok(cn
            .unwrap()
            .is_descendant_of(pn.map(|s| s.hash), &self.nodemap))
    }
    
    // Finds the highest block that is a ancestor to both a and b
    fn highest_common_ancestor(
        &self,
        a: StateHash,
        b: StateHash,
    ) -> Result<StateHash, ForkTreeError> {
        let an = self.get_node(a);
        if an.is_none() {
            return Err(ForkTreeError::NodeNotFound);
        }

        let bn = self.get_node(b);
        if bn.is_none() {
            return Err(ForkTreeError::NodeNotFound);
        }

        // TODO define the impl here.
        let an = an.unwrap();
        let bn = bn.unwrap();

        let mut curr = an;

        loop {
            if curr.hash == bn.hash {
                return Ok(curr.hash);
            }

            if bn.is_descendant_of(Some(curr.hash), &self.nodemap) {
                return Ok(curr.hash);
            }

            // let c = if let Some(c) = curr.parent {c} else {
            //     panic!("failed here");
            // };

            let cur_parent = if let Some(cp) = curr.parent {
                self.nodemap.get(&cp).unwrap()
            } else {
                break;
            };

            curr = cur_parent;
        };

        Err(ForkTreeError::AncestorNotFound)
    }
}

#[cfg(test)]
mod tests {
    use std::{hash::Hash, ops::Deref, os::unix::thread};

    use mina_crypto::hash::{BaseHash, Hashable, StateHash};
    use mina_rs_base::{numbers::Length, protocol_state::ProtocolState};
    use rand::Rng;

    use crate::{BlockTree, Database, ForkTree};

    // TODO: https://github.com/MinaProtocol/mina/blob/29b961e1dae2ffd6eabb8f18d75c4407f74be228/docs/specs/consensus/README.md#27-example-block
    // create a dummy protocol state from the sample block
    // A protocol state object is same as a block header.
    fn dummy_protocol_state(block_number: u32, prev_hash: &StateHash) -> ProtocolState {
        let mut rng = rand::thread_rng();

        let random_genesis_state_hash = StateHash::from_bytes([rng.gen(); 32]);

        let mut ps = ProtocolState {
            previous_state_hash: *prev_hash,
            body: Default::default(),
        };

        ps.body.genesis_state_hash = random_genesis_state_hash;

        ps.body.consensus_state.blockchain_length = Length(block_number);

        ps
    }

    #[derive(Debug)]
    struct TestBranch {
        hash: StateHash,
        depth: u64,
    }

    fn create_test_forktree(
        header: ProtocolState,
        depth: i32,
        db: Database,
    ) -> (ForkTree, Vec<TestBranch>) {
        let mut ft = ForkTree::from_root(&header, db);
        let mut prev_hash = header.hash();

        // branch tree randomly
        let mut branches = vec![];
        let mut rng = rand::thread_rng();

        // create base tree
        for i in 0..depth {
            let header = dummy_protocol_state(i as u32, &prev_hash);
            let hash = header.hash();
            ft.add_block(header, 0).unwrap();
            prev_hash = hash;

            let is_branch = rng.gen_range(0..2);
            if is_branch == 1 {
                branches.push(TestBranch {
                    hash,
                    depth: ft.get_node(hash).unwrap().depth,
                })
            }
        }

        // create tree branches
        for branch in &branches {
            let mut prev_hash = branch.hash;

            for i in branch.depth..depth as u64 {
                let header = dummy_protocol_state(i as u32, &prev_hash);

                let hash = header.hash();
                ft.add_block(header, 0).unwrap();
                prev_hash = hash;
            }
        }

        return (ft, branches);
    }

    fn zero_header() -> ProtocolState {
        dummy_protocol_state(0, &StateHash::from_bytes([0; 32]))
    }

    fn create_flat_tree(depth: i32) -> (ForkTree, Vec<StateHash>) {
        let mut ft = ForkTree::from_root(
            &dummy_protocol_state(0, &&StateHash::from_bytes([0u8; 32])),
            Database,
        );

        let mut prev_hash = ft.root;
        // dbg!("root_hash", prev_hash);
        let mut hashes = vec![ft.root];
        for i in 1..=depth {
            let header = dummy_protocol_state(i as u32, &prev_hash);

            let hash = header.hash();
            hashes.push(hash);
            // dbg!(i);
            // dbg!(header.hash());

            ft.add_block(header, 0).unwrap();
            prev_hash = hash;
        }

        (ft, hashes)
    }

    #[test]
    fn forktree_get_block() {
        let (forktree, hashes) = create_flat_tree(2);

        let n = forktree.get_node(hashes[2]);

        assert!(n.is_some(), "node is null");
        assert_eq!(hashes[2], n.unwrap().hash);
    }

    #[test]
    fn forktree_add_block() {
        let (mut ft, hashes) = create_flat_tree(1);

        let header = dummy_protocol_state(1, &hashes[1]);
        let hash = header.hash();
        ft.add_block(header, 0).unwrap();

        let node = ft.get_node(hash).unwrap();
        let n = ft.leaves.get(node.hash).unwrap();

        let mut a = [0; 32];
        a[1] = 1;

        let old_hash = StateHash::from_bytes(a);

        assert!(ft.leaves.get(old_hash).is_none());
    }

    #[test]
    fn test_is_decendant_of() {
        let (ft, hashes) = create_flat_tree(4);

        assert!(
            ft.is_descendant_of(ft.root, hashes[3]).unwrap(),
            "Failed to verify leaf is descendant of root"
        );

        // Verify the inverse relationship does not hold
        assert!(
            !ft.is_descendant_of(hashes[3], ft.root).unwrap(),
            "root should not be descendant of anything"
        );
    }

    #[test]
    fn test_forktree_longest_path() {
        let (mut ft, hashes) = create_flat_tree(3);

        // insert a block to create a competing path
        let header = dummy_protocol_state(1, &hashes[0]);

        ft.add_block(header, 0).unwrap();

        let longest_path = ft.longest_path();

        for (i, n) in longest_path.iter().enumerate() {
            if *n != hashes[i] {
                panic!("expected hash: {:?} got: {:?}", hashes[i], n);
            }
        }
    }

    #[test]
    fn test_forktree_subchain() {
        let (mut ft, hashes) = create_flat_tree(4);
        let expected_path = &hashes[1..];

        // insert a block to create a competing path
        let extra_block = dummy_protocol_state(1, &hashes[0]);
        ft.add_block(extra_block, 0).unwrap();

        let sub_chain = ft.sub_chain(hashes[1], hashes[3]).unwrap();

        for (i, n) in sub_chain.iter().enumerate() {
            if *n != expected_path[i] {
                panic!("expected hash: {:?}, got: {:?}", expected_path[i], n);
            }
        }
    }

    #[test]
    fn test_forktree_deepest_leaf() {
        let mut arrival_time = 256;
        let mut expected = None;

        let (mut ft, branches) = create_test_forktree(zero_header(), 8, Database);
        let mut deepest = 0;

        let leaves = ft
            .leaves
            .map
            .iter()
            .map(|s| s.clone())
            .collect::<Vec<StateHash>>();

        for node_hash in leaves {
            let mut node = ft.get_node_mut(node_hash).unwrap();
            node.arrival_time = arrival_time;
            arrival_time -= 1;
            if node.depth >= deepest {
                deepest = node.depth;
                expected = Some(node_hash);
            }
        }

        let deepest_leaf = ft.deepest_leaf();
        if deepest_leaf.unwrap() != expected.unwrap() {
            panic!("Fail: got {:?} expected {:?}", deepest_leaf, expected);
        }
    }

    #[test]
    fn test_forktree_get_node() {
        let (mut ft, branches) = create_test_forktree(zero_header(), 15, Database);

        for b in &branches {
            let header = dummy_protocol_state(b.depth as u32, &b.hash);
            ft.add_block(header, 0).unwrap();
        }
    }

    #[test]
    fn test_forktree_get_all_blocks_at_depth() {
        let (ft, branches) = create_test_forktree(zero_header(), 8, Database);
        let root_node = ft.get_node(ft.root).unwrap();
        let mut hashes = root_node.get_nodes_with_depth(10, &ft.nodemap);

        if !hashes.is_empty() {
            panic!("Expected empty array");
        }

        // create one-path tree
        let ft_depth = 8;
        let desired_depth = 6;
        let (mut ft, ft_hashes) = create_flat_tree(ft_depth);

        let mut expected = vec![ft_hashes[desired_depth]];

        // add branch
        let mut prev_hash = ft_hashes[4];

        for i in 4..ft_depth {
            let header = dummy_protocol_state(i as u32, &prev_hash);

            let hash = header.hash();
            ft.add_block(header, 0).unwrap();
            prev_hash = hash;

            if i == desired_depth as i32 - 1 {
                expected.push(hash);
            }
        }

        // add another branch
        prev_hash = ft_hashes[2];

        for i in 2..=ft_depth {
            let header = dummy_protocol_state(i as u32, &prev_hash);
            let hash = header.hash();
            ft.add_block(header, 0).unwrap();
            prev_hash = hash;

            if i == desired_depth as i32 - 1 {
                expected.push(hash);
            }
        }

        let root_node = ft.get_node(ft.root).unwrap();
        hashes = root_node.get_nodes_with_depth(desired_depth as u64, &ft.nodemap);

        assert!(hashes == expected, "Did not get all expected hashes got {:?} expected {:?}", hashes, expected);
    }

    #[test]
    fn test_forktree_is_descendant_of() {
        let (ft, hashes) = create_flat_tree(4);
        let is_descendant = ft.is_descendant_of(ft.root, hashes[3]).unwrap();
        assert!(is_descendant);

        let is_descendant = ft.is_descendant_of(hashes[3], ft.root).unwrap();
        assert!(!is_descendant);
    }


    // FIXME
    // #[test]
    // fn test_forktree_highest_common_ancestor() {
    //     let mut leaves: Vec<StateHash>;
    //     let mut branches: Vec<TestBranch>;
    //     let mut ft: ForkTree;
    //     loop {
    //         let a = create_test_forktree(zero_header(), 8, Database);
    //         ft = a.0;
    //         branches = a.1;

    //         leaves = ft.leaves.leaves();
    //         if leaves.len() == 2 {
    //             break;
    //         };
    //     }

    //     let expected = branches[0].hash;

    //     let a = leaves[0];
    //     let b = leaves[1];

    //     // dbg!(a,b);

    //     let p = ft.highest_common_ancestor(a, b).unwrap();
    //     dbg!(p);

    //     // assert_eq!(p, expected);

    //     // a = leaves
    // }

    #[test]
    fn test_forktree_highest_common_ancestor_same_node() {
        let (ft, _) = create_test_forktree(zero_header(), 8, Database);
        let leaves = ft.leaves.leaves();

        let a = leaves[0];

        let p = ft.highest_common_ancestor(a, a).unwrap();
        assert_eq!(p, a);
    }

    #[test]
    fn test_forktree_highest_common_ancestor_same_chain() {
        let (ft, _) = create_test_forktree(zero_header(), 8, Database);
        let leaves = &ft.leaves;

        let a = leaves.leaves()[0];
        let b = ft.get_node(a).unwrap().parent.unwrap();
        let p = ft.highest_common_ancestor(a, b).unwrap();
    }

    // #[test]
    // fn test_forktree_prune() {
    //     let mut ft: ForkTree;
    //     let mut branches: Vec<TestBranch>;

    //     loop {
    //         let a = create_test_forktree(zero_header(), 5, Database);
    //         ft = a.0;
    //         branches = a.1;
    //         if branches.len() > 0 && ft.get_node(branches[0].hash).unwrap().children.len() > 1 {
    //             break;
    //         }
    //     }

    //     let copy = ft.clone();

    //     // pick some block to finalize
    //     let finalized = ft.get_node(ft.root).unwrap().children[0];
    //     let finalized = ft.get_node(finalized).unwrap().children[0];
    //     let pruned = ft.prune(finalized);

    //     for pruned_hash in pruned {
    //         if copy.is_descendant_of(pruned_hash, finalized).unwrap() {
    //             panic!("pruned node that's descendant of finalised node!!");
    //         }

    //         if copy.is_descendant_of(finalized, pruned_hash).unwrap() {
    //             panic!("pruned an ancestor of the finalised node!!");
    //         }
    //     }

    //     assert!(ft.leaves.len() != 0);

    //     for leaf in ft.leaves.nodes(&ft.nodemap) {
    //         assert_ne!(leaf, finalized);
    //         assert!(ft.is_descendant_of(leaf, finalized).unwrap())
    //     }
    // }

    // #[test]
    // fn test_block_tree_rewind() {
    //     let mut ft: ForkTree;
    //     let mut branches: Vec<TestBranch>;

    //     let rewind = 6;

    //     loop {
    //         let a = create_test_forktree(zero_header(), 12, Database);
    //         ft = a.0;
    //         branches = a.1;
    //         if branches.len() > 0 && ft.get_node(branches[0].hash).unwrap().children.len() > 1 {
    //             break;
    //         }
    //     }

    //     let start = ft.leaves.deepest_leaf(&ft.nodemap).unwrap();
    //     let start_node_depth = { ft.get_node(start).unwrap().depth };

    //     ft.rewind(rewind);
    //     let deepest = ft.leaves.deepest_leaf(&ft.nodemap).unwrap();
    //     let deepest_depth = ft.get_node(deepest).unwrap().depth;
    //     assert!(start_node_depth - rewind as u64 == deepest_depth);
    // }
}
