// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol errors

use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error)]
pub enum SymError {
    /// StackUnderflow
    #[error("Stack underflow")]
    StackUnderflow,
}

/// Result with symbol error
pub type SymResult<T> = Result<T, SymError>;
