// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display trait for tree like output

/// Trait for displaying a tree
pub trait TreeDisplay {
    /// Write item into `f` and use `{:depth$}` syntax in front of your single line
    /// output to get proper indention.
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result;

    /// Display as tree starting at depth `0`.
    fn display_tree(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.tree_print(
            f,
            TreeState {
                depth: 0,
                debug: false,
            },
        )
    }

    /// Display as tree starting at given depth in debug mode
    fn debug_tree(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.tree_print(
            f,
            TreeState {
                depth: 0,
                debug: true,
            },
        )
    }

    /// Display as tree starting at depth `0` into a file
    fn write_tree(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(
            f,
            "{}",
            WriteFmt(|f| self.tree_print(
                f,
                TreeState {
                    depth: 0,
                    debug: false
                }
            ))
        )
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
    /// Print in debug mode
    pub debug: bool,
}

impl TreeState {
    /// Create new tree state
    /// - `shorten`: If `true` content will be shortened to one line
    pub fn new_display() -> Self {
        Self {
            depth: 0,
            debug: false,
        }
    }

    pub fn new_debug(depth: usize) -> Self {
        Self { depth, debug: true }
    }
    /// Change indention one step deeper
    pub fn indent(&mut self) {
        self.depth += INDENT
    }

    /// Return a indention which is one step deeper
    pub fn indented(&self) -> Self {
        Self {
            depth: self.depth + INDENT,
            debug: self.debug,
        }
    }
}

/// print syntax via std::fmt::Display
pub struct FormatTree<'a, T: TreeDisplay>(pub &'a T);

impl<T: TreeDisplay> std::fmt::Display for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_print(
            f,
            TreeState {
                depth: 2,
                debug: false,
            },
        )
    }
}

impl<T: TreeDisplay> std::fmt::Debug for FormatTree<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tree_print(
            f,
            TreeState {
                depth: 2,
                debug: true,
            },
        )
    }
}
