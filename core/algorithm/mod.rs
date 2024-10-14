// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

pub mod boolean_op;

use crate::{
    render::ModelNode,
    Result,
};

pub use boolean_op::BooleanOp;

/// Algorithm trait
pub trait Algorithm: std::fmt::Debug {
    /// Processes geometry for a 2d renderer and returns a geometry
    fn process_2d(&self, _renderer: &mut crate::Renderer2D, _parent: ModelNode) -> Result<crate::geo2d::Node> {
        unimplemented!()
    }

    /// Processes geometry for a 3d renderer and returns a geometry
    fn process_3d(&self, _renderer: &mut crate::Renderer3D, _parent: ModelNode) -> Result<crate::geo3d::Node> {
        unimplemented!()
    }
}
