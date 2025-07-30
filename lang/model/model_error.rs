// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value errors.

use crate::model::*;
use thiserror::Error;

/// Value error
#[derive(Debug, Error)]
pub enum ModelError {
    /// Invalid operator
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
}

pub(crate) type ModelResult<Type = Model> = std::result::Result<Type, ModelError>;
