// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Line and column within a source code file
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LineCol {
    /// Line number (1..)
    pub line: usize,
    /// Column number (1..)
    pub col: usize,
}

impl std::fmt::Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
