// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, objects::*, rc::*, resolve::*, syntax::*, ty::*};

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
        vec![Parameter::no_ref("radius", Type::Scalar)].into()
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

#[derive(Debug)]
struct Rect {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
}

impl BuiltinModuleDefinition for Rect {
    fn id() -> &'static str {
        "rect"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode> {
        Ok(ObjectNode::new(ObjectNodeInner::Primitive2D(Rc::new(
            Rect {
                width: args.get_value::<Scalar>(&Identifier::no_ref("width")),
                height: args.get_value::<Scalar>(&Identifier::no_ref("height")),
                x: args.get_value::<Scalar>(&Identifier::no_ref("x")),
                y: args.get_value::<Scalar>(&Identifier::no_ref("y")),
            },
        ))))
    }

    fn parameters() -> ParameterList {
        vec![
            Parameter::no_ref("width", Type::Scalar),
            Parameter::no_ref("height", Type::Scalar),
            Parameter::no_ref("x", Type::Scalar),
            Parameter::no_ref("y", Type::Scalar),
        ]
        .into()
    }
}

impl microcad_core::RenderHash for Rect {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo2d::Primitive for Rect {
    fn render_geometry(
        &self,
        _renderer: &mut dyn geo2d::Renderer,
    ) -> microcad_core::CoreResult<geo2d::Geometry> {
        use geo::line_string;

        // Create a rectangle from the given width, height, x and y
        let line_string = line_string![
            (x: self.x, y: self.y),
            (x: self.x + self.width, y: self.y),
            (x: self.x + self.width, y: self.y + self.height),
            (x: self.x, y: self.y + self.height),
            (x: self.x, y: self.y),
        ];

        Ok(geo2d::Geometry::MultiPolygon(
            microcad_core::geo2d::line_string_to_multi_polygon(line_string),
        ))
    }
}

/// Builtin module for 2D geometry
pub fn geo2d() -> Symbol {
    crate::NamespaceBuilder::new("geo2d".try_into().expect("valid id"))
        .symbol(Circle::symbol())
        .symbol(Rect::symbol())
        .build()
}
