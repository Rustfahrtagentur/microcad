// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree iterators

use std::collections::VecDeque;

use super::*;

/// Children iterator struct.
pub struct Children {
    node: ModelNode,
    index: usize,
}

impl Children {
    /// Create new [`Children`] iterator
    pub fn new(node: ModelNode) -> Self {
        Self { node, index: 0 }
    }
}

impl Iterator for Children {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.borrow();
        let child = node.children().get(self.index);
        self.index += 1;
        child.cloned()
    }
}

/// Iterator over all descendants.
pub struct Descendants {
    stack: std::collections::VecDeque<ModelNode>,
}

impl Descendants {
    /// Create new descendants iterator
    pub fn new(root: ModelNode) -> Self {
        Self {
            stack: root
                .borrow()
                .children()
                .iter()
                .cloned()
                .collect::<VecDeque<_>>(),
        }
    }
}

impl Iterator for Descendants {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop a node from the front of the stack
        if let Some(node) = self.stack.pop_front() {
            // Push this node's children to the front of the stack (DFS)
            let children = node.borrow().children().clone();
            for child in children.iter().rev() {
                self.stack.push_front(child.clone());
            }
            Some(node)
        } else {
            None
        }
    }
}
