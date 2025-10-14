// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use thiserror::Error;

/// Resolve error.
#[derive(Debug, Error)]
pub enum DiagError {
    /// Cannot continue evaluation after error limit has been reached.
    #[error("Error limit reached: Stopped evaluation after {0} errors")]
    ErrorLimitReached(u32),
}

/// Result type of any resolve.
pub type DiagResult<T> = std::result::Result<T, DiagError>;
