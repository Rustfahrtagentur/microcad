// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Quantity Error module

use thiserror::Error;

use crate::value::Quantity;

/// Error when processing a [`Quantity`].
#[derive(Debug, Error)]
pub enum QuantityError {
    /// Error adding two quantities.
    #[error("Invalid operation: `{0}` {1} `{2}` (try `{2}` {1} `{0}`)")]
    InvalidOperation(Quantity, char, Quantity),
}

/// Quality result type.
pub type QuantityResult = Result<Quantity, QuantityError>;
