mod triangle_mesh;

pub use manifold_rs::Manifold;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

pub enum Geometry {
    Mesh(TriangleMesh),
    Manifold(Manifold),
}
