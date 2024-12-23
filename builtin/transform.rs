// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cgmath::Vector3;
use microcad_lang::{builtin_module, eval::*, objects::*, parse::*, sym::*};

use crate::namespace_builder::NamespaceBuilder;
use microcad_core::Scalar;

/// Built-in transformations
fn translate(x: Scalar, y: Scalar, z: Scalar) -> Result<ObjectNode, EvalError> {
    Ok(ObjectNode::new(ObjectNodeInner::Transform(
        Transform::Translation(microcad_core::Vec3::new(x, y, z)),
    )))
}

fn rotate(angle: Scalar, x: Scalar, y: Scalar, z: Scalar) -> Result<ObjectNode, EvalError> {
    Ok(ObjectNode::new(ObjectNodeInner::Transform(
        Transform::Rotation(cgmath::Rad(angle), Vector3::new(x, y, z)),
    )))
}

pub fn builtin_namespace() -> std::rc::Rc<NamespaceDefinition> {
    NamespaceBuilder::new("transform")
        .add(builtin_module!(rotate(angle: Scalar, x: Scalar, y: Scalar, z: Scalar)).into())
        .add(builtin_module!(translate(x: Scalar, y: Scalar, z: Scalar)).into())
        .build()
}
