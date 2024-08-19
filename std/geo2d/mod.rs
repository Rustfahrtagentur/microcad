use microcad_builtin_proc_macro::DefineBuiltInModule;
use microcad_core::Scalar;
use microcad_parser::{
    eval::Symbols,
    language::{
        lang_type::Type,
        module::{DefineBuiltInModule, ModuleDefinition},
        parameter::ParameterList,
    },
};
use microcad_render::geo2d::{Generator, Geometry, LineString};

#[derive(DefineBuiltInModule)]
pub struct Circle {
    pub radius: Scalar,
}

impl Generator for Circle {
    fn generate(
        &self,
        renderer: &dyn microcad_render::Renderer,
        _: microcad_render::tree::Node,
    ) -> Geometry {
        let mut points = Vec::new();
        use std::f64::consts::PI;

        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u64;

        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            points.push(geo::coord!(x: self.radius * angle.cos(), y: self.radius * angle.sin()));
        }

        Geometry::MultiPolygon(microcad_render::geo2d::line_string_to_multi_polygon(
            LineString::new(points),
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

impl Generator for Rect {
    fn generate(
        &self,
        _: &dyn microcad_render::Renderer,
        _: microcad_render::tree::Node,
    ) -> Geometry {
        use geo::line_string;

        // Create a rectangle from the given width, height, x and y
        let line_string = line_string![
            (x: self.x, y: self.y),
            (x: self.x + self.width, y: self.y),
            (x: self.x + self.width, y: self.y + self.height),
            (x: self.x, y: self.y + self.height),
            (x: self.x, y: self.y),
        ];

        Geometry::MultiPolygon(microcad_render::geo2d::line_string_to_multi_polygon(
            line_string,
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
