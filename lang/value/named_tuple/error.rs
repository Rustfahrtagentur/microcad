// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple error
use thiserror::Error;

use crate::value::*;

/// Value error
#[derive(Debug, Error)]
pub enum TupleError {
    /// Cannot convert to color.
    #[error("Cannot convert named tuple to color: {0}")]
    CannotConvertToColor(Tuple),
}
