// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD Renderer

pub mod render2d;
pub use render2d::{Primitive2D, Renderer2D};

pub mod render3d;
pub use render3d::{Primitive3D, Renderer3D};

pub mod tree;
pub use tree::{ModelNode, ModelNodeInner};

/// Render hash trait
pub trait RenderHash {
    /// Calculate a hash of self
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

/// Renderer trait
pub trait Renderer {
    /// The precision of the renderer in mm
    fn precision(&self) -> crate::Scalar;

    /// Change the render state
    fn change_render_state(&mut self, _: &str, _: &str) -> crate::Result<()> {
        Ok(())
    }
}
