// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node output types.

use std::rc::Rc;

use microcad_core::{Geometry2D, Geometry3D, RenderResolution};
use strum::IntoStaticStr;

use crate::model_tree::ModelNode;



#[derive(Debug, Clone)]
pub struct ModelNodeOutput2D {
    geometries: Vec<Rc<Geometry2D>>,
    matrix: microcad_core::Mat3,
    bounds: microcad_core::geo2d::Bounds,
    resolution: RenderResolution,
}

#[derive(Debug, Clone)]
pub struct ModelNodeOutput3D {
    geometries: Vec<Rc<Geometry3D>>,
    matrix: microcad_core::Mat4,
    bounds: microcad_core::geo3d::Bounds,
    resolution: RenderResolution,
}

/// The output type of the [`ModelNode`].
#[derive(Debug, Clone, IntoStaticStr, Default, PartialEq)]
pub enum ModelNodeOutput {
    /// The output type has not yet been determined.
    #[default]
    NotDetermined,

    /// The [`ModelNode`] outputs a 2d geometry.
    Geometry2D(ModelNodeOutput2D),

    /// The [`ModelNode`] outputs a 3d geometry.
    Geometry3D(ModelNodeOutput3D),

    /// The [`ModelNode`] is invalid, you cannot mix 2d and 3d geometry.
    Invalid,
}

impl ModelNodeOutput {


    fn calculate_matrix(&mut self, node: ModelNode) {

        match node.parent() {

        }
    }

    fn calculate_bounds(&mut self)

}

pub trait ModelNodeOutputAccess {
    fn add_geometry_2d(&mut self, geometry: Rc<Geometry2D>);


}



impl std::fmt::Display for ModelNodeOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}
