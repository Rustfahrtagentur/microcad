// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

pub mod boolean_op;

use crate::{
    render::{Node, Renderer2D, Renderer3D},
    Result,
};

pub use boolean_op::BooleanOp;

/// Algorithm trait
pub trait Algorithm {
    fn process_2d(&self, _renderer: &mut dyn Renderer2D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }

    fn process_3d(&self, _renderer: &mut dyn Renderer3D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }
}
