// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display trait for tree like output

/// Trait for displaying a tree
pub trait TreeDisplay {
    /// Write item into `f` and use `{:depth$}` syntax in front of your single line
    /// output to get proper indention.
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result;

    /// Display as tree starting at depth `0`.
    fn print_tree(&self, f: &mut std::fmt::Formatter, shorten: bool) -> std::fmt::Result {
        self.tree_print(f, TreeState { depth: 0, shorten })
    }

    /// Display as tree starting at depth `0`.
    fn write_tree(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(f, "{}", WriteFmt(|f| self.print_tree(f, false)))
    }
}

/// Helper to write into io from fmt writers
struct WriteFmt<F>(pub F)
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result;

impl<F> std::fmt::Display for WriteFmt<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}

/// Indention size
const INDENT: usize = 2;

/// Indention depth counter
#[derive(derive_more::Deref, Clone, Copy)]
pub struct TreeState {
    #[deref]
    depth: usize,
    /// Data shall be shortened to one-line if `true`
    pub shorten: bool,
}

impl TreeState {
    /// Create new tree state
    /// - `shorten`: If `true` content will be shortened to one line
    pub fn new(shorten: bool) -> Self {
        Self { depth: 0, shorten }
    }

    /// Change indention one step deeper
    pub fn indent(&mut self) {
        self.depth += INDENT
    }

    /// Return a indention which is one step deeper
    pub fn indented(&self) -> Self {
        Self {
            depth: self.depth + INDENT,
            shorten: self.shorten,
        }
    }
}

impl From<usize> for TreeState {
    fn from(depth: usize) -> Self {
        TreeState {
            depth,
            shorten: true,
        }
    }
}

/// print syntax via std::fmt::Display
pub struct FormatTree<'a, T: TreeDisplay>(pub &'a T);

impl<T: TreeDisplay> std::fmt::Display for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_print(f, 2.into())
    }
}
