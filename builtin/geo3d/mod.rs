// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, objects::*, rc::*, resolve::Symbol, syntax::*, ty::Type};

/// The builtin sphere primitive, defined by its radius.
#[derive(Debug)]
pub struct Sphere {
    /// Radius of the sphere in millimeters
    pub radius: Scalar,
}

impl BuiltinModuleDefinition for Sphere {
    fn name() -> &'static str {
        "sphere"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode> {
        Ok(ObjectNode::new(ObjectNodeInner::Primitive3D(Rc::new(
            Sphere {
                radius: args.get_value::<Scalar>(&Identifier::no_ref("radius")),
            },
        ))))
    }

    fn parameters() -> ParameterList {
        vec![Parameter::no_ref("radius", Type::Scalar)].into()
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


impl BuiltinModuleDefinition for Cube {
    fn name() -> &'static str {
        "cube"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode> {
        Ok(ObjectNode::new(ObjectNodeInner::Primitive3D(Rc::new(
            Cube {
                size_x: args.get_value::<Scalar>(&Identifier::no_ref("size_x")),
                size_y: args.get_value::<Scalar>(&Identifier::no_ref("size_y")),
                size_z: args.get_value::<Scalar>(&Identifier::no_ref("size_z")),
            },
        ))))
    }

    fn parameters() -> ParameterList {
        vec![
            Parameter::no_ref("size_x", Type::Scalar),
            Parameter::no_ref("size_y", Type::Scalar),
            Parameter::no_ref("size_z", Type::Scalar),
        ].into()
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

impl BuiltinModuleDefinition for Cylinder {
    fn name() -> &'static str {
        "cylinder"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode> {
        Ok(ObjectNode::new(ObjectNodeInner::Primitive3D(Rc::new(
            Cylinder {
                radius_bottom: args.get_value::<Scalar>(&Identifier::no_ref("radius_bottom")),
                radius_top: args.get_value::<Scalar>(&Identifier::no_ref("radius_top")),
                height: args.get_value::<Scalar>(&Identifier::no_ref("height")),
            },
        ))))
    }

    fn parameters() -> ParameterList {
        vec![
            Parameter::no_ref("radius_bottom", Type::Scalar),
            Parameter::no_ref("radius_top", Type::Scalar),
            Parameter::no_ref("height", Type::Scalar),
        ].into()
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

/// geo3d Builtin module
pub fn geo3d() -> Symbol {
    crate::NamespaceBuilder::new("geo3d".try_into().expect("valid id"))
        .symbol(Sphere::symbol())
        .symbol(Cube::symbol())
        .symbol(Cylinder::symbol())
        .build()
}
