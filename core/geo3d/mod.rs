mod triangle_mesh;

pub use manifold_rs::Manifold;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

pub enum Geometry {
    Mesh(TriangleMesh),
    Manifold(Manifold),
}

impl Geometry {
    pub fn fetch_mesh(&self) -> TriangleMesh {
        match self {
            Geometry::Mesh(mesh) => mesh.clone(),
            Geometry::Manifold(manifold) => TriangleMesh::from(manifold.mesh()),
        }
    }
}

impl From<Manifold> for Geometry {
    fn from(manifold: Manifold) -> Self {
        Geometry::Manifold(manifold)
    }
}

impl From<TriangleMesh> for Geometry {
    fn from(mesh: TriangleMesh) -> Self {
        Geometry::Mesh(mesh)
    }
}
