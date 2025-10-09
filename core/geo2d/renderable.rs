// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Trait for 2D Renderable.

use std::rc::Rc;

use super::Geometry2D;
use crate::RenderResolution;

/// Something that can rendered into a 2D geometry with a certain resolution.
pub trait Renderable2D {
    /// Render self into some Geometry with a certain render resolution
    ///
    /// Note: We might want to have [`RenderCache`] as argument here, hence we return an `Rc`.
    fn render_to_geometry(&self, resolution: &RenderResolution) -> Rc<Geometry2D>;
}

impl Renderable2D for Rc<Geometry2D> {
    fn render_to_geometry(&self, _: &RenderResolution) -> Rc<Geometry2D> {
        self.clone()
    }
}
