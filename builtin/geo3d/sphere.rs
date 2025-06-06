// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, objects::*, rc::*, syntax::*, ty::Type};

/// The builtin sphere primitive, defined by its radius.
#[derive(Debug)]
pub struct Sphere {
    /// Radius of the sphere in millimeters
    pub radius: Scalar,
}

impl BuiltinPartDefinition for Sphere {
    fn id() -> &'static str {
        "sphere"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(
            Element::Primitive3D(Rc::new(Sphere {
                radius: args.get_value::<Scalar>(&Identifier::no_ref("radius")),
            })),
        ))
    }

    fn parameters() -> ParameterList {
        ParameterList::new(vec![Parameter::no_ref("radius", Type::Scalar)].into())
    }
}

impl RenderHash for Sphere {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo3d::Primitive for Sphere {
    fn render_geometry(
        &self,
        renderer: &mut dyn geo3d::Renderer,
    ) -> microcad_core::CoreResult<geo3d::Geometry> {
        use std::f64::consts::PI;
        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u32;

        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::sphere(
            self.radius,
            n,
        )))
    }
}
