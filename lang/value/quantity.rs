// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Matrix value type

use crate::{model_tree::*, syntax::*, ty::*};
use microcad_core::*;

/// A numeric value
#[derive(Clone, PartialEq)]
pub enum Quantity {
    /// A unitless scalar value.
    Scalar(Scalar),
    /// Length in mm.
    Length(Scalar),
    /// Area in mm².
    Area(Scalar),
    /// Volume in mm³.
    Volume(Scalar),
    /// Density in g/mm³
    Density(Scalar),
    /// An angle in radians.
    Angle(Scalar),
    /// Weight of a specific volume of material in g.
    Weight(Scalar),
}

impl From<Scalar> for Quantity {
    fn from(value: Scalar) -> Self {
        Self::Scalar(value)
    }
}
