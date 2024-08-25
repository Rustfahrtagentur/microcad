pub use manifold_rs::{Manifold, Mesh};

pub enum Geometry {
    Mesh(Mesh),
    Manifold(Manifold),
}
