// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Measure node attribute.

use crate::value::Value;

/// Measure attribute.
#[derive(Clone, Debug, Default)]
pub enum MeasureAttribute {
    /// Measure the size of a geometry (for each dimension).
    #[default]
    Size,
}

impl From<MeasureAttribute> for Value {
    fn from(_: MeasureAttribute) -> Self {
        Value::String("size".into())
    }
}
