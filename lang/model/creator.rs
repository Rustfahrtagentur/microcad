// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Creator of work pieces.

use crate::{resolve::Symbol, value::Tuple};

/// A creator is the origin  
#[derive(Debug, Clone)]
pub struct Creator {
    /// Workpiece arguments.
    pub arguments: Tuple,
    /// Symbol.
    pub symbol: Symbol,
}

impl Creator {
    pub fn new(arguments: Tuple, symbol: Symbol) -> Self {
        Self { arguments, symbol }
    }
}
