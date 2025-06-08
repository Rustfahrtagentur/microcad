// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree iterators

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
    stack: Vec<Children>,
}

impl Descendants {
    /// Create new descendants iterator
    pub fn new(root: ModelNode) -> Self {
        Self {
            stack: vec![Children::new(root)],
        }
    }
}

impl Iterator for Descendants {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let child = {
                let children = self.stack.last_mut().expect("Non-empty stack");
                children.next()
            };

            match &child {
                Some(child) => {
                    if child.has_children() {
                        self.stack.push(child.children());
                    }
                }
                None => {
                    self.stack.pop();
                }
            }

            child
        }
    }
}
