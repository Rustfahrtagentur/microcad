use microcad_builtin_proc_macro::DefineBuiltInModule;
use microcad_core::geo2d::{Geometry, LineString};
use microcad_core::Scalar;
use microcad_parser::{eval::*, language::*};
use microcad_render::{RenderHash, Renderable2D};

#[derive(DefineBuiltInModule)]
pub struct Circle {
    pub radius: Scalar,
}

impl RenderHash for Circle {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl Renderable2D for Circle {
    fn render_geometry(
        &self,
        renderer: &mut dyn microcad_render::Renderer2D,
    ) -> microcad_core::Result<Geometry> {
        let mut points = Vec::new();
        use std::f64::consts::PI;

        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u64;

        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            points.push(geo::coord!(x: self.radius * angle.cos(), y: self.radius * angle.sin()));
        }

        Ok(Geometry::MultiPolygon(
            microcad_core::geo2d::line_string_to_multi_polygon(LineString::new(points)),
        ))
    }
}

use microcad_render::tree::{Node, NodeInner};

#[derive(DefineBuiltInModule)]
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

impl Renderable2D for Rect {
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
use crate::ModuleBuilder;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("geo2d")
        .add_builtin_module(Circle::builtin_module())
        .add_builtin_module(Rect::builtin_module())
        .build()
}
