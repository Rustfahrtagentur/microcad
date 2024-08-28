pub mod boolean_op;

use crate::{
    render::{Node, Renderer2D, Renderer3D},
    Result,
};

pub use boolean_op::BooleanOp;

pub trait Algorithm {
    fn process_2d(&self, _renderer: &mut dyn Renderer2D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }

    fn process_3d(&self, _renderer: &mut dyn Renderer3D, _parent: Node) -> Result<Node> {
        unimplemented!()
    }
}
