use cxx::{let_cxx_string, CxxString};

#[cxx::bridge(namespace = "manifold_rs")]
mod ffi {

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("manifold_rs.h");

        type Manifold;

        fn sphere(radius: f64) -> UniquePtr<Manifold>;
        fn cube(x_size: f64, y_size: f64, z_size: f64) -> UniquePtr<Manifold>;

        fn mesh_from_manifold(manifold: &Manifold) -> UniquePtr<Mesh>;

        type Material;

        //fn material() -> UniquePtr<Material>;

        type Mesh;

        fn export_mesh(filename: &CxxString, mesh: &Mesh);
        //fn import_mesh(filename: &str) -> UniquePtr<Mesh>;
    }
}

#[test]
fn test_manifold() {
    let sphere = ffi::sphere(1.0);

    let mesh = ffi::mesh_from_manifold(&sphere);

    let_cxx_string!(filename = "sphere.obj");
    ffi::export_mesh(&filename, &mesh);
}
