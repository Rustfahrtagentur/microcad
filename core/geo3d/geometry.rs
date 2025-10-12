// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::*;

use std::rc::Rc;
use strum::IntoStaticStr;

use crate::geo3d::*;

/// 3D Geometry
#[derive(IntoStaticStr, Clone)]
pub enum Geometry3D {
    /// Triangle mesh.
    Mesh(TriangleMesh),
    /// Manifold.
    Manifold(Rc<Manifold>),
    /// Collection.
    Collection(Geometries3D),
}

impl std::fmt::Debug for Geometry3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

impl Geometry3D {
    /// Return name of geometry.
    pub fn name(&self) -> &'static str {
        self.into()
    }

    /// Execute boolean operation.
    pub fn boolean_op(&self, other: &Geometry3D, op: &BooleanOp) -> Option<Self> {
        let op: manifold_rs::BooleanOp = op.into();
        let a: Rc<Manifold> = self.clone().into();
        let b: Rc<Manifold> = other.clone().into();
        Some(Geometry3D::Manifold(Rc::new(a.boolean_op(&b, op))))
    }

    /// Calculate contex hull.
    pub fn hull(&self) -> Self {
        match &self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.to_manifold().hull().into(),
            Geometry3D::Manifold(manifold) => manifold.hull().into(),
            Geometry3D::Collection(collection) => {
                TriangleMesh::from(collection).to_manifold().hull().into()
            }
        }
    }
}

impl FetchBounds3D for Rc<Manifold> {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        todo!()
    }
}

impl FetchBounds3D for Geometry3D {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        match self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.fetch_bounds_3d(),
            Geometry3D::Manifold(manifold) => manifold.fetch_bounds_3d(),
            Geometry3D::Collection(collection) => collection.fetch_bounds_3d(),
        }
    }
}

impl Transformed3D for Geometry3D {
    fn transformed_3d(&self, mat: &Mat4) -> Self {
        TriangleMesh::from(self.clone()).transformed_3d(mat).into()
    }
}

impl From<TriangleMesh> for Geometry3D {
    fn from(mesh: TriangleMesh) -> Self {
        Geometry3D::Mesh(mesh)
    }
}

impl From<Manifold> for Geometry3D {
    fn from(manifold: Manifold) -> Self {
        Geometry3D::Manifold(Rc::new(manifold))
    }
}

impl From<Rc<Manifold>> for Geometry3D {
    fn from(manifold: Rc<Manifold>) -> Self {
        Geometry3D::Manifold(manifold)
    }
}

impl From<Geometry3D> for Rc<Manifold> {
    fn from(geo: Geometry3D) -> Self {
        match geo {
            Geometry3D::Mesh(triangle_mesh) => Rc::new(triangle_mesh.to_manifold()),
            Geometry3D::Manifold(manifold) => manifold,
            Geometry3D::Collection(ref collection) => {
                Rc::new(TriangleMesh::from(collection).to_manifold())
            }
        }
    }
}
