use microcad_core::Scalar;

pub struct Circle {
    pub radius: Scalar,
}

use microcad_parser::language::module::{BuiltinModule, ModuleDefinition};
use microcad_render::geo2d::{Generator, Geometry, LineString};

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

use crate::ModuleBuilder;

pub fn circle(radius: Scalar) -> Node {
    Node::new(NodeInner::Generator2D(Box::new(Circle { radius })))
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("geo2d")
        .builtin_module(BuiltinModule {
            name: "circle".into(),
            f: &|args, ctx| {
                if let Ok(arg) = args.arg_1("radius") {
                    ctx.append_node(circle(arg.into_scalar().unwrap()));
                }
            },
        })
        .build()
}
