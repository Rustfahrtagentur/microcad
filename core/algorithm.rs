use crate::render::render2d::Renderer2D;
use crate::render::Node;

use crate::*;

pub trait Algorithm {
    fn process_2d(&self, _renderer: &mut dyn Renderer2D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }
    /*     fn process_3d(
        &self,
        renderer: &dyn Renderer3D,
        parent: Node,
    ) -> Result<Box<dyn Renderable3D>, Error> {
        unimplemented!()
    }*/
}
