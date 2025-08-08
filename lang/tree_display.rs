// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display trait for tree like output

/// Trait for displaying a tree
pub trait TreeDisplay {
    /// Write item into [`f`] and use `{:depth$}` syntax in front of your single line
    /// output to get proper indention.
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeIndent) -> std::fmt::Result;
}

/// Indention size
const INDENT: usize = 2;

/// Indention depth counter
#[derive(derive_more::Deref, Clone, Copy)]
pub struct TreeIndent(usize);

impl TreeIndent {
    /// Change indention one step deeper
    pub fn indent(&mut self) {
        self.0 += INDENT
    }

    /// Return a indention which is one step deeper
    pub fn indented(&self) -> Self {
        Self(self.0 + INDENT)
    }
}

impl From<usize> for TreeIndent {
    fn from(depth: usize) -> Self {
        TreeIndent(depth)
    }
}
