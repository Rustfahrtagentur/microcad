// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macro::DefineBuiltinPrimitive2D;
use microcad_core::{
    geo2d::{Geometry, LineString},
    Scalar,
};
use microcad_lang::{eval::*, parse::*};
use microcad_render::{RenderHash, Primitive2D};

#[derive(DefineBuiltinPrimitive2D, Debug)]
pub struct Circle {
    pub radius: Scalar,
}

impl RenderHash for Circle {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl Primitive2D for Circle {
    fn render_geometry(
        &self,
        renderer: &mut dyn microcad_render::Renderer2D,
    ) -> microcad_core::Result<Geometry> {
        use std::f64::consts::PI;

        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u64;

        let range = 0..n;
        let points = range
            .map(|i| {
                let angle = 2.0 * PI * (i as f64) / (n as f64);
                geo::coord!(x: self.radius * angle.cos(), y: self.radius * angle.sin())
            })
            .collect();

        Ok(Geometry::MultiPolygon(
            microcad_core::geo2d::line_string_to_multi_polygon(LineString::new(points)),
        ))
    }
}

#[derive(DefineBuiltinPrimitive2D, Debug)]
struct Rect {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
}

impl RenderHash for Rect {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl Primitive2D for Rect {
    fn render_geometry(
        &self,
        _renderer: &mut dyn microcad_render::Renderer2D,
    ) -> microcad_core::Result<Geometry> {
        use geo::line_string;

        // Create a rectangle from the given width, height, x and y
        let line_string = line_string![
            (x: self.x, y: self.y),
            (x: self.x + self.width, y: self.y),
            (x: self.x + self.width, y: self.y + self.height),
            (x: self.x, y: self.y + self.height),
            (x: self.x, y: self.y),
        ];

        Ok(Geometry::MultiPolygon(
            microcad_core::geo2d::line_string_to_multi_polygon(line_string),
        ))
    }
}

use crate::NamespaceBuilder;

pub fn builtin_module() -> std::rc::Rc<NamespaceDefinition> {
    NamespaceBuilder::new("geo2d")
        .add(Circle::builtin_module().into())
        .add(Rect::builtin_module().into())
        .build()
}
