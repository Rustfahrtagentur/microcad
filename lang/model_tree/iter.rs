// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree iterators

use crate::syntax::SourceFile;

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
    stack: ModelNodes,
}

impl Descendants {
    /// Create new descendants iterator
    pub fn new(root: ModelNode) -> Self {
        Self {
            stack: root
                .borrow()
                .children()
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl Iterator for Descendants {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let children = node.borrow().children().clone();
            for child in children.iter().rev() {
                self.stack.push(child.clone());
            }
            Some(node)
        } else {
            None
        }
    }
}

/// Iterator over all parents of a [`ModelNode`].
pub struct Parents {
    node: Option<ModelNode>,
}

impl Parents {
    /// New parents iterator
    pub fn new(node: ModelNode) -> Self {
        Self { node: Some(node) }
    }
}

impl Iterator for Parents {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.node {
            Some(node) => {
                self.node = node.parent();
                self.node.clone()
            }
            None => None,
        }
    }
}

/// Iterator over all ancestors (this node and its parents)
pub struct Ancestors {
    node: Option<ModelNode>,
}

impl Ancestors {
    /// New parents iterator
    pub fn new(node: ModelNode) -> Self {
        Self { node: Some(node) }
    }
}

impl Iterator for Ancestors {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = match &self.node {
            Some(node) => node.clone(),
            None => return None,
        };

        self.node = node.parent();
        Some(node.clone())
    }
}

/// Iterator over all descendants.
pub struct SourceFileDescendants {
    stack: ModelNodes,
    source_file: std::rc::Rc<SourceFile>,
}

impl SourceFileDescendants {
    /// Create a new source file descendants.
    pub fn new(root: ModelNode) -> Self {
        let source_file = root.find_source_file().expect("A source file");

        Self {
            stack: root
                .borrow()
                .children()
                .filter_by_source_file(&source_file.clone())
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .into(),
            source_file,
        }
    }
}

impl Iterator for SourceFileDescendants {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let children = node
                .borrow()
                .children()
                .filter_by_source_file(&self.source_file);
            for child in children.iter().rev() {
                self.stack.push(child.clone());
            }

            Some(node)
        } else {
            None
        }
    }
}
