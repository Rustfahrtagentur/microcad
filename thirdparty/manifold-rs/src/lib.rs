use cxx::{let_cxx_string, CxxVector};

#[cxx::bridge(namespace = "manifold_rs")]
mod ffi {

    // C++ types and signatures exposed to Rust.

    unsafe extern "C++" {
        include!("manifold_rs.h");

        type Manifold;

        fn sphere(radius: f64) -> UniquePtr<Manifold>;
        fn cube(x_size: f64, y_size: f64, z_size: f64) -> UniquePtr<Manifold>;

        type Mesh;

        fn mesh_from_manifold(manifold: &Manifold) -> UniquePtr<Mesh>;
        fn mesh_vertices(mesh: &Mesh) -> UniquePtr<CxxVector<f32>>;
        fn mesh_indices(mesh: &Mesh) -> UniquePtr<CxxVector<u32>>;
    }
}

#[test]
fn test_manifold() {
    let sphere = ffi::sphere(1.0);

    let mesh = ffi::mesh_from_manifold(&sphere);

    let vertices_binding = ffi::mesh_vertices(&mesh);
    let vertices = Box::new(vertices_binding.as_ref().unwrap().as_slice());
    assert!(!vertices.is_empty());

    let indices_binding = ffi::mesh_indices(&mesh);
    let indices: Box<_> = Box::new(indices_binding.as_ref().unwrap().as_slice());
    assert!(!indices.is_empty());
}
