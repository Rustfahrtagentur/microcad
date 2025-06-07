// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, rc::*, src_ref::*, syntax::*, ty::Type};
/// The built-in cylinder primitive, defined by an bottom radius, top radius and height.
/// The cylinder is oriented along the z-axis.
#[derive(Debug)]
pub struct Cylinder {
    /// Bottom radius of the cylinder in millimeters
    pub radius_bottom: Scalar,
    /// Top radius of the cylinder in millimeters
    pub radius_top: Scalar,
    /// Height of the cylinder in millimeters
    pub height: Scalar,
}

impl BuiltinPartDefinition for Cylinder {
    fn id() -> &'static str {
        "cylinder"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(Refer::none(Element::Primitive3D(
            Rc::new(Cylinder {
                radius_bottom: args.get_value::<Scalar>(&Identifier::no_ref("radius_bottom")),
                radius_top: args.get_value::<Scalar>(&Identifier::no_ref("radius_top")),
                height: args.get_value::<Scalar>(&Identifier::no_ref("height")),
            }),
        ))))
    }

    fn parameters() -> ParameterList {
        ParameterList::new(
            vec![
                Parameter::no_ref("radius_bottom", Type::Scalar),
                Parameter::no_ref("radius_top", Type::Scalar),
                Parameter::no_ref("height", Type::Scalar),
            ]
            .into(),
        )
    }
}

impl RenderHash for Cylinder {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo3d::Primitive for Cylinder {
    fn render_geometry(
        &self,
        renderer: &mut dyn geo3d::Renderer,
    ) -> microcad_core::CoreResult<geo3d::Geometry> {
        use std::f64::consts::PI;
        let n = (self.radius_bottom / renderer.precision() * PI * 0.5).max(3.0) as u32;

        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::cylinder(
            self.radius_bottom,
            self.radius_top,
            self.height,
            n,
        )))
    }
}
