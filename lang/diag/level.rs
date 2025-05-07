// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// The level of the diagnostic.
///
/// Levels have a priority in order.
#[derive(Debug, Clone)]
pub enum Level {
    /// Trace message (highest diagnosis level).
    Trace,
    /// Informative message.
    Info,
    /// Warning message.
    Warning,
    /// Error (lowest diagnosis level).
    Error,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Trace => write!(f, "trace"),
            Level::Error => write!(f, "error"),
            Level::Warning => write!(f, "warning"),
            Level::Info => write!(f, "info"),
        }
    }
}
