// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod render2d;
pub use render2d::{Renderable2D, Renderer2D};

pub mod render3d;
pub use render3d::{Renderable3D, Renderer3D};

pub mod tree;
pub use tree::{Node, NodeInner};

pub trait RenderHash {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

pub trait Renderer {
    // The precision of the renderer in mm
    fn precision(&self) -> crate::Scalar;

    // Change the render state
    fn change_render_state(&mut self, _: &str, _: &str) -> crate::Result<()> {
        Ok(())
    }
}

