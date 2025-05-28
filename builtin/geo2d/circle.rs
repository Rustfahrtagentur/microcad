// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, objects::*, rc::*, syntax::*, ty::*};

/// Builtin definition for a 2D circle
#[derive(Debug)]
pub struct Circle {
    /// Radius of the circle in millimeters
    pub radius: Scalar,
}

impl BuiltinModuleDefinition for Circle {
    fn id() -> &'static str {
        "circle"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode> {
        Ok(ObjectNode::new(ObjectNodeInner::Primitive2D(Rc::new(
            Circle {
                radius: args.get_value::<Scalar>(&Identifier::no_ref("radius")),
            },
        ))))
    }

    fn parameters() -> ParameterList {
        ParameterList::new(vec![Parameter::no_ref("radius", Type::Scalar)].into())
    }
}

impl microcad_core::RenderHash for Circle {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo2d::Primitive for Circle {
    fn render_geometry(
        &self,
        renderer: &mut dyn geo2d::Renderer,
    ) -> microcad_core::CoreResult<geo2d::Geometry> {
        use std::f64::consts::PI;

        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u64;

        let range = 0..n;
        let points = range
            .map(|i| {
                let angle = 2.0 * PI * (i as f64) / (n as f64);
                geo::coord!(x: self.radius * angle.cos(), y: self.radius * angle.sin())
            })
            .collect();

        Ok(geo2d::Geometry::MultiPolygon(
            microcad_core::geo2d::line_string_to_multi_polygon(geo2d::LineString::new(points)),
        ))
    }
}
