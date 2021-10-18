// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use dashmap::{DashMap, DashSet};
use mina_crypto::hash::StateHash;

use crate::node::Node;

// LeafMap provides a quick lookup for existing leaves
// which do not have any children yet.
#[derive(Clone)]
pub(crate) struct LeafMap {
    pub(crate) map: DashSet<StateHash>,
}

impl LeafMap {
    pub(crate) fn new() -> Self {
        Self {
            map: DashSet::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.map.len()
    }

    pub(crate) fn leaves(&self) -> Vec<StateHash> {
        self.map.iter().map(|s| s.clone()).collect()
    }

    pub(crate) fn nodes(&self, nodemap: &DashMap<StateHash, Node>) -> Vec<StateHash> {
        let a = nodemap.clone().into_read_only();
        let a: Vec<StateHash> = a.keys().map(|s| s.clone()).collect();
        a
    }

    // deepest_leaf searches the stored leaves to the find the one with the greatest depth.
    // If there are two leaves with the same depth, choose the one with the earliest arrival time.
    pub(crate) fn deepest_leaf(&self, nodemap: &DashMap<StateHash, Node>) -> Option<StateHash> {
        let mut max = 0;
        let mut d_leaf = StateHash::default();

        for k in self.map.iter() {
            d_leaf = *k;
            let node = nodemap.get(&k).unwrap();
            if max < node.depth {
                max = node.depth;
                d_leaf = *k;
            } else if max == node.depth
                && node.arrival_time < {
                    let node = self.map.get(&d_leaf);
                    let node = node.as_deref().unwrap();
                    let node = nodemap.get(node).unwrap();
                    node.arrival_time
                }
            {
                d_leaf = *k;
            }
        }

        return self.map.get(&d_leaf).as_deref().cloned();
    }

    // replace deletes the old node from the map and inserts the new one at its place.
    pub(crate) fn replace(&mut self, old_node: &StateHash, new_node: StateHash) {
        self.map.remove(&old_node);
        self.map.insert(new_node);
    }

    pub(crate) fn get(&self, hash: StateHash) -> Option<StateHash> {
        self.map.get(&hash).as_deref().cloned()
    }

    pub(crate) fn put(&mut self, hash: StateHash) {
        self.map.insert(hash);
    }
}
