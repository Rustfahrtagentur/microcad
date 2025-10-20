// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Creator of work pieces.

use crate::{
    resolve::{FullyQualify, Symbol},
    value::Tuple,
};

/// A creator is the origin  
#[derive(Debug, Clone)]
pub struct Creator {
    /// Symbol.
    pub symbol: Symbol,
    /// Workpiece arguments.
    pub arguments: Tuple,
}

impl Creator {
    /// New creator.
    pub fn new(symbol: Symbol, arguments: Tuple) -> Self {
        Self { symbol, arguments }
    }
}

impl std::fmt::Display for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{symbol}{arguments}",
            symbol = self.symbol.full_name(),
            arguments = self.arguments
        )
    }
}

impl std::hash::Hash for Creator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.full_name().hash(state);
        self.arguments.hash(state);
    }
}
