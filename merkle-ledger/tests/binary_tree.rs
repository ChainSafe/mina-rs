// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use merkle_ledger::traits;

trait Account {
    type T;
}

enum T<H: traits::hash::Hash + Clone> {
    Node(Box<Node<H>>),
    Leaf(H),
}

struct Node<H: traits::hash::Hash + Clone> {
    pub hash: H,
    pub left: T<H>,
    pub right: T<H>,
}

struct Make<A: Account, H: traits::hash::Hash + Clone> {
    _a: PhantomData<A>,
    _b: PhantomData<H>,
}

impl<A: Account, H> Make<A, H>
where
    H: traits::hash::Hash + Clone,
{
    pub fn get_hash(other: &T<H>) -> H {
        match &*other {
            T::Leaf(h) => h.clone(),
            T::Node(n) => n.hash.clone(),
        }
    }

    fn go(list: &[H], num_nodes: usize) -> (T<H>, &[H], usize) {
        if num_nodes == 1 {
            if list.len() != 0 {
                return (T::Leaf(list[0].clone()), &list[1..], 0);
            } else {
                panic!("Expected to recurse on a non-empty list");
            }
        } else {
            let (left_tree, right_list, left_height) = Make::<A, H>::go(list, num_nodes / 2);
            let (right_tree, remaining_nodes, right_height) =
                Make::<A, H>::go(right_list, num_nodes / 2);
            assert_eq!(left_height, right_height);
            let hash = H::merge(
                left_height,
                &Make::<A, H>::get_hash(&left_tree),
                &Make::<A, H>::get_hash(&right_tree),
            );
            (
                T::Node(Box::new(Node::<H> {
                    hash,
                    left: left_tree,
                    right: right_tree,
                })),
                remaining_nodes,
                left_height + 1,
            )
        }
    }

    pub fn set_accounts(list: &[A]) {}
}
