// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Matrix type

/// M x N Matrix Type.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatrixType {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub columns: usize,
}

impl MatrixType {
    /// Create new matrix type with rows and columns
    pub fn new(rows: usize, columns: usize) -> Self {
        Self { rows, columns }
    }
}

impl std::fmt::Display for MatrixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix{}x{}", self.rows, self.columns)
    }
}
