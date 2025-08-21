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

    /// Width command
    Width,

    /// Height command
    Height,
}

impl From<MeasureCommand> for Value {
    fn from(command: MeasureCommand) -> Self {
        match command {
            MeasureCommand::Size => Value::String("size".into()),
            MeasureCommand::Width => Value::String("width".into()),
            MeasureCommand::Height => Value::String("height".into()),
        }
    }
}
