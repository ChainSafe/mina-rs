// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    child_nodes: Option<BoxedNode<T>>,
}
type BoxedNode<T> = Box<Node<T>>;

impl<T: std::fmt::Debug> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            child_nodes: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.child_nodes.is_none()
    }

    fn add_child_nodes(&mut self, new_node: Node<T>) {
        if self.is_leaf() {
            self.child_nodes = Some(Box::new(new_node));
        } else {
            self.child_nodes.as_mut().unwrap().add_child_nodes(new_node);
        }
    }

    fn height(&self) -> usize {
        if self.is_leaf() {
            return 0;
        }
        return 1 + self.child_nodes.as_ref().unwrap().height();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tree() {
        let mut root = Node::new(1);
        root.add_child_nodes(Node::new(2));
        root.add_child_nodes(Node::new(3));
        root.add_child_nodes(Node::new(4));
        root.add_child_nodes(Node::new(5));
        assert_eq!(root.data, 1);
        assert_eq!(root.height(), 4);
    }
}
