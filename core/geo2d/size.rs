// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Scalar;

/// 2D size in millimeters.  
#[derive(Clone, Default)]
pub struct Size2D {
    /// Width in mm.
    pub width: Scalar,
    /// Height in mm.
    pub height: Scalar,
}

impl Size2D {
    /// A4 sheet.
    pub const A4: Size2D = Size2D {
        width: 210.0,
        height: 297.0,
    };
}
