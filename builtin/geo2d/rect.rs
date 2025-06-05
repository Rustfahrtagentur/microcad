// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, objects::*, rc::*, syntax::*, ty::*};

#[derive(Debug)]
pub struct Rect {
    width: Scalar,
    height: Scalar,
    x: Scalar,
    y: Scalar,
}

impl BuiltinPartDefinition for Rect {
    fn id() -> &'static str {
        "rect"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode> {
        Ok(ObjectNode::new_from_content(
            ObjectNodeContent::Primitive2D(Rc::new(Rect {
                width: args.get_value::<Scalar>(&Identifier::no_ref("width")),
                height: args.get_value::<Scalar>(&Identifier::no_ref("height")),
                x: args.get_value::<Scalar>(&Identifier::no_ref("x")),
                y: args.get_value::<Scalar>(&Identifier::no_ref("y")),
            })),
        ))
    }

    fn parameters() -> ParameterList {
        ParameterList::new(
            vec![
                Parameter::no_ref("width", Type::Scalar),
                Parameter::no_ref("height", Type::Scalar),
                Parameter::no_ref("x", Type::Scalar),
                Parameter::no_ref("y", Type::Scalar),
            ]
            .into(),
        )
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
