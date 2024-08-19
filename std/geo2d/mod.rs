use microcad_core::Scalar;
use microcad_parser::{
    builtin_module,
    eval::Symbols,
    language::{
        lang_type::Type,
        module::{BuiltinModule, ModuleDefinition},
        parameter::Parameter,
    },
};
use microcad_render::geo2d::{Generator, Geometry, LineString};

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

pub fn circle(radius: Scalar) -> Node {
    Node::new(NodeInner::Generator2D(Box::new(Circle { radius })))
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Generator for Rectangle {
    fn generate(
        &self,
        _: &dyn microcad_render::Renderer,
        _: microcad_render::tree::Node,
    ) -> Geometry {
        use geo::line_string;
        let w2 = self.width / 2.0;
        let h2 = self.height / 2.0;
        // Rect is centered at 0,0
        let line_string = line_string![
            (x: -w2, y: -h2),
            (x: w2, y: -h2),
            (x: w2, y: h2),
            (x: -w2, y: h2),
            (x: -w2, y: -h2),
        ];

        Geometry::MultiPolygon(microcad_render::geo2d::line_string_to_multi_polygon(
            line_string,
        ))
    }
}

pub fn rect(width: f64, height: f64) -> Node {
    Node::new(NodeInner::Generator2D(Box::new(Rectangle {
        width,
        height,
    })))
}

use crate::ModuleBuilder;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("geo2d")
        .add_builtin_module(builtin_module!(circle(radius: Scalar)))
        .add_builtin_module(builtin_module!(rect(width: Scalar, height: Scalar)))
        .build()
}
