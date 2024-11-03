// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Rust integration of C++ library *Manifold* for geometric operations

#[cxx::bridge(namespace = "manifold_rs")]
mod ffi {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("manifold_rs.h");

        type Manifold;

        fn sphere(radius: f64, segments: u32) -> UniquePtr<Manifold>;
        fn cube(x_size: f64, y_size: f64, z_size: f64) -> UniquePtr<Manifold>;
        fn cylinder(
            radius_low: f64,
            radius_high: f64,
            height: f64,
            segments: u32,
        ) -> UniquePtr<Manifold>;

        fn union_(a: &Manifold, b: &Manifold) -> UniquePtr<Manifold>;
        fn intersection(a: &Manifold, b: &Manifold) -> UniquePtr<Manifold>;
        fn difference(a: &Manifold, b: &Manifold) -> UniquePtr<Manifold>;

        type Mesh;

        fn vertices(self: &Mesh) -> UniquePtr<CxxVector<f32>>;
        fn indices(self: &Mesh) -> UniquePtr<CxxVector<u32>>;

        fn mesh_from_manifold(manifold: &Manifold) -> UniquePtr<Mesh>;
        fn manifold_from_mesh(mesh: &Mesh) -> UniquePtr<Manifold>;

        fn mesh_from_vertices(vertices: &[f32], indices: &[u32]) -> UniquePtr<Mesh>;
    }
}

pub enum BooleanOp {
    Union,
    Intersection,
    Difference,
}

/// Manifold rust wrapper for C++ manifold object.
pub struct Manifold(cxx::UniquePtr<ffi::Manifold>);

impl Manifold {
    /// Create a sphere manifold.
    pub fn sphere(radius: f64, segments: u32) -> Self {
        Self(ffi::sphere(radius, segments))
    }

    /// Create a cube manifold.
    pub fn cube(x_size: f64, y_size: f64, z_size: f64) -> Self {
        Self(ffi::cube(x_size, y_size, z_size))
    }

    /// Create a cylinder manifold.
    pub fn cylinder(radius_low: f64, radius_high: f64, height: f64, segments: u32) -> Self {
        Self(ffi::cylinder(radius_low, radius_high, height, segments))
    }

    /// Get the union of two manifolds.
    pub fn union(&self, b: &Self) -> Self {
        Self(ffi::union_(self.inner(), b.inner()))
    }

    /// Get the intersection of two manifolds.
    pub fn intersection(&self, b: &Self) -> Self {
        Self(ffi::intersection(self.inner(), b.inner()))
    }

    /// Get the difference of two manifolds.
    pub fn difference(&self, b: &Self) -> Self {
        Self(ffi::difference(self.inner(), b.inner()))
    }

    pub fn boolean_op(&self, b: &Self, op: crate::BooleanOp) -> Self {
        match op {
            crate::BooleanOp::Union => self.union(b),
            crate::BooleanOp::Intersection => self.intersection(b),
            crate::BooleanOp::Difference => self.difference(b),
        }
    }

    /// Get the mesh representation of the manifold.
    pub fn to_mesh(&self) -> Mesh {
        Mesh(ffi::mesh_from_manifold(&self.0))
    }

    pub fn from_mesh(mesh: Mesh) -> Self {
        mesh.into()
    }

    /// Get the inner C++ manifold object.
    fn inner(&self) -> &ffi::Manifold {
        self.0.as_ref().unwrap()
    }
}

pub struct Mesh(cxx::UniquePtr<ffi::Mesh>);

impl Mesh {
    pub fn new(vertices: &[f32], indices: &[u32]) -> Self {
        let mesh = ffi::mesh_from_vertices(vertices, indices);
        Self(mesh)
    }

    pub fn vertices(&self) -> Vec<f32> {
        let vertices_binding = self.0.vertices();
        let vertices = vertices_binding.as_ref().unwrap().as_slice();
        vertices.to_vec()
    }

    pub fn indices(&self) -> Vec<u32> {
        let indices_binding = self.0.indices();
        let indices = indices_binding.as_ref().unwrap().as_slice();
        indices.to_vec()
    }

    pub fn manifold(&self) -> Manifold {
        let manifold = ffi::manifold_from_mesh(&self.0);
        Manifold(manifold)
    }
}

impl From<Mesh> for Manifold {
    fn from(mesh: Mesh) -> Self {
        mesh.manifold()
    }
}

impl From<Manifold> for Mesh {
    fn from(manifold: Manifold) -> Self {
        manifold.to_mesh()
    }
}

#[test]
fn test_manifold_ffi() {
    let sphere = ffi::sphere(1.0, 32);

    let mesh = ffi::mesh_from_manifold(&sphere);

    let vertices_binding = mesh.vertices();
    let vertices = vertices_binding.as_ref().unwrap().as_slice();
    assert!(!vertices.is_empty());

    let indices_binding = mesh.indices();
    let indices = indices_binding.as_ref().unwrap().as_slice();
    assert!(!indices.is_empty());
}
