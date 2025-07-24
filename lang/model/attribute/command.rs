// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::model::{ExportCommand, MeasureCommand};

/// Command attribute, e.g.: `#[export: "test.svg"]`.
pub enum CommandAttribute {
    /// Export attribute: `#[export: "test.svg"`.
    Export(ExportCommand),

    /// Measure attribute: `#[measure: width, height]`.
    Measure(MeasureCommand),
}
