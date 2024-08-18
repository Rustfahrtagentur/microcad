pub mod geo2d;
pub mod svg;
pub mod tree;

use microcad_core::Scalar;
use tree::Node;

pub trait Renderer {
    /// Precision in mm
    fn precision(&self) -> Scalar;

    fn render(&mut self, node: Node);
}
