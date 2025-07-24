// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Measure model attribute.

use crate::value::Value;

/// Measure attribute.
#[derive(Clone, Debug, Default)]
pub enum MeasureCommand {
    /// Measure the size of a geometry (for each dimension).
    #[default]
    Size,
}

impl From<MeasureCommand> for Value {
    fn from(_: MeasureCommand) -> Self {
        Value::String("size".into())
    }
}
