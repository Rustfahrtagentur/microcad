// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, objects::*, rc::*, syntax::*, ty::Type};

/// The builtin cube primitive, defined by its size in the x, y, and z dimensions.
#[derive(Debug)]
pub struct Cube {
    /// Size of the cube in the x dimension in millimeters
    pub size_x: Scalar,
    /// Size of the cube in the y dimension in millimeters
    pub size_y: Scalar,
    /// Size of the cube in the z dimension in millimeters
    pub size_z: Scalar,
}

impl BuiltinPartDefinition for Cube {
    fn id() -> &'static str {
        "cube"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(Element::Primitive3D(Rc::new(
            Cube {
                size_x: args.get_value::<Scalar>(&Identifier::no_ref("size_x")),
                size_y: args.get_value::<Scalar>(&Identifier::no_ref("size_y")),
                size_z: args.get_value::<Scalar>(&Identifier::no_ref("size_z")),
            },
        ))))
    }

    fn parameters() -> ParameterList {
        ParameterList::new(
            vec![
                Parameter::no_ref("size_x", Type::Scalar),
                Parameter::no_ref("size_y", Type::Scalar),
                Parameter::no_ref("size_z", Type::Scalar),
            ]
            .into(),
        )
    }
}

impl RenderHash for Cube {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo3d::Primitive for Cube {
    fn render_geometry(
        &self,
        _renderer: &mut dyn geo3d::Renderer,
    ) -> microcad_core::CoreResult<geo3d::Geometry> {
        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::cube(
            self.size_x,
            self.size_y,
            self.size_z,
        )))
    }
}
