
use dashmap::DashMap;
use mina_crypto::hash::StateHash;
use crate::error::ForkTreeError;

use crate::{NodeRef, node};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
    pub(crate) hash: StateHash, // FIXME is this correct?
    pub(crate) parent: Option<StateHash>, // FIXME: maybe switch to vector indices, if multiple ownership is required
    pub(crate) children: Vec<StateHash>,
    pub(crate) depth: u64,
    pub(crate) arrival_time: u64 // Arrival time of the block in unix epoch
}

impl Node {
    // adds the given node's hash to its children
    pub(crate) fn add_child(&mut self, node_hash: StateHash) {
        self.children.push(node_hash);
    }

    // get_nodes_with_depth returns all descendent nodes with the desired depth
    pub(crate) fn get_nodes_with_depth(&self, depth: u64, nodemap: &DashMap<StateHash, Node>) -> Vec<StateHash> {
        let mut hashes = vec![];
        for node in &self.children {
            let node = nodemap.get(node).unwrap(); // Todo: Remove unwrap()
            if node.depth == depth {
                hashes.push(node.hash.clone());
            }
            if node.depth > depth {
                return hashes;
            }

            hashes.extend_from_slice(&node.get_nodes_with_depth(depth, nodemap));
        }

        hashes
    }

    // FIXME: Mostly unoptimized, keeping this way until we freeze on
    // the use case.
    pub(crate) fn sub_chain(&self, descendant: Option<&Node>, nodemap: &DashMap<StateHash, Node>) -> Result<Vec<StateHash>, ForkTreeError> {
        // if descendant 
        if let Some(d) = descendant {
            let mut path = vec![];
            if self.hash == d.hash {
                path.push(self.hash);
            }

            let mut curr = if let Some(ph) = d.parent {
                nodemap.get(&ph)
                // crate::get_node(&ph)
            } else {
                return Ok(path);
            };

            while let Some(cur) = &curr {
                // TODO: gossamer code seems to return things in
                // reverse order of appends. i.e., first element is parent
                // Is that necessary?
                let c = cur;
                path.push(c.hash);
                if c.hash == self.hash {
                    path.reverse();
                    return Ok(path);
                }

                if let Some(n) = &cur.parent {
                    curr = nodemap.get(&n);
                } else {
                    break;
                }
            }

            Ok(path)
        } else {
            return Err(ForkTreeError::DescendantNotFound);
        }
    }

    // is_descendant_of traverses the tree following all possible paths until it determines if self is a descendant of parent
    pub(crate) fn is_descendant_of(&self, parent: Option<StateHash>, nodemap: &DashMap<StateHash, Node>) -> bool {
        if let Some(p) = parent {
            let parent = nodemap.get(&p).unwrap();
            if self.depth < parent.depth {
                // node's depth is small than parent, bail.
                return false;
            }

            let mut self_parent = self.parent;
            loop {
                if let Some(self_par) = self_parent {
                    if let Some(self_par) = nodemap.get(&self_par) {
                        if self_par.depth == parent.depth {
                            return true;
                        } else {
                            self_parent = self_par.parent;
                        }
                    }
                } else {
                    break;
                }
            }
        } else {
            return false;
        }

        false
    }

    // get_Leaves returns all nodes that are leaf nodes with the current node as its ancestor
    pub(crate) fn get_leaves(&self, nodemap: &DashMap<StateHash, Node>) -> Vec<StateHash> {
        let mut leaves = vec![];
        if self.children.is_empty() {
            // FIXME: currently clones the nodes
            leaves.push(self.hash);
        }

        for c in &self.children {
            let child_node = nodemap.get(c).unwrap();
            let ch_leaves = child_node.get_leaves(nodemap);
            leaves.extend_from_slice(&ch_leaves);
        }

        leaves
    }

    fn get_all_descendants(&self, nodemap: &DashMap<StateHash, Node>) -> Vec<StateHash> {
        let mut hashes = vec![];
        hashes.push(self.hash.clone());

        for c in &self.children {
            // TODO remove unwrap
            let node = nodemap.get(c).unwrap();
            hashes.extend(node.get_all_descendants(nodemap))
        }

        hashes
    }

    pub fn delete_child(&mut self, to_delete: StateHash) {
        for i in 0..self.children.len() {
            if self.children[i] == to_delete {
                // crate
                self.children.remove(i);
            }
        }
    }

    pub(crate) fn prune(&self, finalized: &Node, nodemap: &DashMap<StateHash, Node>) -> Vec<StateHash> {
        let mut pruned = vec![];
        // if this is a descedent of the finalised block, keep it
        // all descendents of this block will also be descendents of the finalised block,
        // so don't need to check any of those
        if self.is_descendant_of(Some(finalized.hash), nodemap) {
            return pruned;
        // if it's not an ancestor of the finalised block, prune it
        } else if !finalized.is_descendant_of(Some(self.hash), nodemap) {
            pruned.push(self.hash.clone());
        }

        // if this is an ancestor of the finalised block, keep it,
	    // and check its children
        for c in &self.children {
            let child_node = nodemap.get(c).unwrap();
            let child_pruned = child_node.prune(finalized, nodemap);
            pruned.extend_from_slice(&child_pruned);
        }

        pruned
    }
}
