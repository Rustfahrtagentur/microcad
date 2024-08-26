use crate::render::Node;
use crate::render::{Renderer2D, Renderer3D};

use crate::*;

pub trait Algorithm {
    fn process_2d(&self, _renderer: &mut dyn Renderer2D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }

    fn process_3d(&self, _renderer: &mut dyn Renderer3D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }
}
