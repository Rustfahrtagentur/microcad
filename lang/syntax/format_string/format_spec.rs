// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format Specification syntax element

use crate::{src_ref::*, syntax::*};

/// Format specification
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FormatSpec {
    /// Precision for number formatting
    pub precision: Option<u32>,
    /// Alignment width (leading zeros)
    pub width: Option<u32>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for FormatSpec {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for FormatSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.width, self.precision) {
            (Some(width), Some(precision)) => write!(f, "0{width}.{precision}"),
            (None, Some(precision)) => write!(f, ".{precision}"),
            (Some(width), None) => write!(f, "0{width}"),
            _ => Ok(()),
        }
    }
}

impl TreeDisplay for FormatSpec {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeIndent) -> std::fmt::Result {
        match (self.width, self.precision) {
            (Some(width), Some(precision)) => {
                writeln!(f, "{:depth$}FormatSpec: 0{width}.{precision}", "")
            }
            (None, Some(precision)) => writeln!(f, "{:depth$}FormatSpec: .{precision}", ""),
            (Some(width), None) => writeln!(f, "{:depth$}FormatSpec:  0{width}", ""),
            _ => Ok(()),
        }
    }
}
