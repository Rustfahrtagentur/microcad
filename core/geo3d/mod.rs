mod triangle_mesh;

pub use manifold_rs::Manifold;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

use crate::algorithm::BooleanOp;

pub enum Geometry {
    Mesh(TriangleMesh),
    Manifold(Manifold),
}

impl From<&BooleanOp> for manifold_rs::BooleanOp {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Union => manifold_rs::BooleanOp::Union,
            BooleanOp::Intersection => manifold_rs::BooleanOp::Intersection,
            BooleanOp::Difference => manifold_rs::BooleanOp::Difference,
            _ => unimplemented!(),
        }
    }
}

impl Geometry {
    pub fn fetch_mesh(&self) -> TriangleMesh {
        match self {
            Geometry::Mesh(mesh) => mesh.clone(),
            Geometry::Manifold(manifold) => TriangleMesh::from(manifold.to_mesh()),
        }
    }

    pub fn boolean_op(&self, other: &Geometry, op: &BooleanOp) -> Option<Self> {
        let op: manifold_rs::BooleanOp = op.into();
        match (self, other) {
            (Geometry::Mesh(a), Geometry::Mesh(b)) => {
                let result = a.to_manifold().boolean_op(&b.to_manifold(), op);
                Some(Geometry::Manifold(result))
            }
            (Geometry::Manifold(a), Geometry::Manifold(b)) => {
                let result: Manifold = a.boolean_op(b, op);
                Some(Geometry::Manifold(result))
            }
            (Geometry::Mesh(a), Geometry::Manifold(b)) => {
                let result = a.to_manifold().boolean_op(b, op);
                Some(Geometry::Manifold(result))
            }
            (Geometry::Manifold(a), Geometry::Mesh(b)) => {
                let result = a.boolean_op(&b.to_manifold(), op);
                Some(Geometry::Manifold(result))
            }
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
