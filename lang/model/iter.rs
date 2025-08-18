// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree iterators

use super::*;

/// Children iterator struct.
pub struct Children {
    model: Model,
    index: usize,
}

impl Children {
    /// Create new [`Children`] iterator
    pub fn new(model: Model) -> Self {
        Self { model, index: 0 }
    }
}

impl Iterator for Children {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.model.borrow();
        let child = model.children.get(self.index);
        self.index += 1;
        child.cloned()
    }
}

/// Iterator over all descendants.
pub struct Descendants {
    stack: Models,
}

impl Descendants {
    /// Create new descendants iterator
    pub fn new(root: Model) -> Self {
        Self {
            stack: root
                .borrow()
                .children
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl Iterator for Descendants {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(model) = self.stack.pop() {
            let children = model.borrow().children.clone();
            for child in children.iter().rev() {
                self.stack.push(child.clone());
            }
            Some(model)
        } else {
            None
        }
    }
}

/// Iterator over all parents of a [`Model`].
pub struct Parents {
    model: Option<Model>,
}

impl Parents {
    /// New parents iterator
    pub fn new(model: Model) -> Self {
        Self { model: Some(model) }
    }
}

impl Iterator for Parents {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.model {
            Some(model) => {
                let parent = model.borrow().parent.clone();
                self.model = parent;
                self.model.clone()
            }
            None => None,
        }
    }
}

/// Iterator over all ancestors (this model and its parents)
pub struct Ancestors {
    model: Option<Model>,
}

impl Ancestors {
    /// New parents iterator
    pub fn new(model: Model) -> Self {
        Self { model: Some(model) }
    }
}

impl Iterator for Ancestors {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        let model = match &self.model {
            Some(model) => model.clone(),
            None => return None,
        };

        self.model = model.borrow().parent.clone();
        Some(model.clone())
    }
}

/// Iterator over all descendants.
pub struct SourceFileDescendants {
    stack: Models,
    source_file_hash: u64,
}

impl SourceFileDescendants {
    /// Create a new source file descendants.
    pub fn new(root: Model) -> Self {
        let source_file_hash = root.borrow().origin.src_ref.source_hash();

        Self {
            stack: root
                .borrow()
                .children
                .filter_by_source_hash(source_file_hash)
                .iter()
                .rev()
                .cloned()
                .collect(),
            source_file_hash,
        }
    }
}

impl Iterator for SourceFileDescendants {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(model) = self.stack.pop() {
            let children = model
                .borrow()
                .children
                .filter_by_source_hash(self.source_file_hash);
            for child in children.iter().rev() {
                self.stack.push(child.clone());
            }

            Some(model)
        } else {
            None
        }
    }
}
