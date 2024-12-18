// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Line and column within a source code file
#[derive(Clone, Debug, Default)]
pub struct LineCol {
    /// Line number (1..)
    pub line: u32,
    /// Column number (1..)
    pub col: u32,
}

impl std::fmt::Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
