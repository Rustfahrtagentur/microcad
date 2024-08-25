pub mod render2d;
pub use render2d::{Renderable2D, Renderer2D};

pub mod tree;
pub use tree::{Node, NodeInner};

pub trait RenderHash {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}
